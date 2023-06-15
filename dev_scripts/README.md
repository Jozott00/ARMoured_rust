
# Dev Scripts

## Template Script
The template script generates a instruction type template with the complete api documentation.

E.g. if we want to implement the instructions for `Load/store memory tags`, just copy the link 
`https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldsttags` and pass it as argument to the script.
```bash
python template.py https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding/Loads-and-Stores?lang=en#ldsttags 
```

Copy the printed output and paste it in a new rust file.