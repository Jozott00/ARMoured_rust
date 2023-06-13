#!/bin/sh python

# This script takes a valid instruction type link such as
# https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Data-Processing----Register?lang=en#log_shift
# and generates a new template for this instruction type, where the docs are already done for you

import sys

import requests
import re
import base64
from lxml import html, etree
import os
import urllib.parse

doc_base_url = 'https://documentation-service.arm.com'
dev_base_url = 'https://developer.arm.com'

doc_prefix = "/// "
mod_doc_prefix = "//! "


def main():
    if len(sys.argv) != 2:
        print(f"Usage: {os.path.basename(__file__)} <doc_link>")
        exit(1)

    url = urllib.parse.urlparse(sys.argv[1])

    type_name = extract_name_from_url(sys.argv[1])
    type_id = url.fragment
    if type_id == None:
        print("link must be of specific instruction type (fragment link)")
        exit(1)

    print(f"Create template for '{type_name}'...\n\n")

    content = get_content(url.path)

    types = content.xpath(f'//*[@id="iclass-{type_id}"]')

    first = types[0]
    instrs = first.xpath('.//td/a[1]')

    # element is {full_name, url}
    unique_instrs = {}

    # filter unique instrs
    for instr in instrs:
        name = instr.text
        if name not in unique_instrs:
            unique_instrs[name] = construct_instruction_definition(instr)

    mod_doc = f"""
# {type_name}

Implements the following instructions:
""".strip()

    method_defs = []

    for name, obj in unique_instrs.items():
        mod_doc += f"\n - [{obj['name']}]({dev_base_url + obj['url']})"
        for var in obj['variants']:
            method_defs.append(create_meth_def(obj, var))

    trait_doc = prepend_per_line(mod_doc, doc_prefix)
    mod_doc = prepend_per_line(mod_doc, mod_doc_prefix)

    final_str = ""
    final_str += mod_doc
    final_str += "\n\n\n\n"
    final_str += trait_doc
    final_str += "\npub trait MyTrait<T>: InstructionProcessor<T> {"
    final_str += "\n\n\n\n"

    for i, m_def in enumerate(method_defs):
        final_str += m_def
        final_str += f"\nfn f{i}(&mut self, ) -> T" + " { \ntodo!()\n\n} \n\n\n"

    final_str += "}"

    print(final_str)


def create_meth_def(obj, var):
    str = f"""
[{obj['name']}]({dev_base_url + obj['url']})

{obj['info']}

```asm
{var}
```
""".strip()
    return prepend_per_line(str, doc_prefix)


def prepend_per_line(str, prefix):
    lines = str.splitlines()
    lines = [prefix + line for line in lines]
    return '\n'.join(lines)


def construct_instruction_definition(instr):
    url = instr.get('href')
    # Extract the last path in URL before the query parameters start
    last_path = re.search(r'.*/(.*?)-\?', url).group(1)
    # Replace '--' with ' - ', remove trailing '-', and replace '-' with ' '
    full_name = last_path.replace('--', 'PLACEHOLDER').rstrip('-').replace('-', ' ').replace('PLACEHOLDER', ' - ')
    info_variants = extract_info_and_variants(url)
    return {'name': full_name, 'url': url, 'info': info_variants['info'], 'variants': info_variants['variants']}


def extract_name_from_url(url):
    # Extract the last path in URL before the query parameters start
    last_path = re.search(r'.*/(.*?)\?', url).group(1)

    # Temporarily replace '----' and '--' with unique placeholders, remove trailing '-'
    # replace remaining '-' with ' ', restore placeholders
    clean_string = last_path.replace('----', 'PLACEHOLDER1').replace('--', 'PLACEHOLDER2').rstrip('-').replace('-', ' ')
    return clean_string.replace('PLACEHOLDER1', ' (').replace('PLACEHOLDER2', ' - ') + ")"


def extract_info_and_variants(url):
    content = get_content(url)
    infos = content.xpath('/html/body/p[1] | /html/body/p[2]')
    info = '\n\n'.join(list(map(lambda x: x.xpath('string()'), infos))).strip()

    variants = []
    for elem in content.xpath('/html/body/div/p'):
        variants.append(elem.xpath('string()'))

    return {'info': info, 'variants': variants}


def get_content(url):
    content = html.fromstring(get_raw_html(url))
    return content


def get_raw_html(url):
    url = f"{doc_base_url}{url}"
    page = requests.get(url)
    json = page.json()
    return base64.b64decode(json['content'])


if __name__ == '__main__':
    main()

    # print(get_raw_html('/documentation/ddi0596/2021-12/Base-Instructions/CRC32B--CRC32H--CRC32W--CRC32X--CRC32-checksum-?lang=en').decode('utf-8'))
    print()
