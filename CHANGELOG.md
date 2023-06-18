## v1.0.0 (2023-06-18)

### BREAKING CHANGE

- main

### Fix

- **mapped-errors**: replace function with-expected-false to with-expected-true of base mapped-error

## v0.6.9 (2023-04-25)

### BREAKING CHANGE

- main

### Feat

- convert the code storage from a single error to multiple errors
- implements method to check if a list of codes contains the mapped-error code
- implements a function to check if a code exists in mapped-error
- implements the display drrive to error code
- create an addicional setter for expected value of mapped-errors
- update mapped-errors struct to include granular creation of them
- include getters for the mapped-error code attribute
- upgrade the message mapped-errors default error manager to dealing with standardized error codes
- create result factories to mapped errors and refactor dtos exports
- include serde serialize and deserialize derives in mapped errors
- create a paginated option for the find many response
- implements the standard error to mapped errors struct
- create a new entity type for multiple depetions
- initial commit

### Fix

- receipt codes in mapped-error as str instead of string
- fix message parsing during mapped-errors construction
- remove as error true and move expected error logger from default to as-error method
- fix the missing import of the mapped-error object from utils

### Refactor

- rename has-any-of-codes function to is-in
- rename parent and children to remove enum sufixes
