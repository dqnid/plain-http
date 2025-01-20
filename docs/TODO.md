# TODO

## V0

- [x] Manage requests
- [NOT NEEDED] HTTP code mapper : not needed because a code will be directly translated on the client (browser / postman)

## V1

- [x] Router
- [ ] JS / CSS data
 - [ ] Automatically expose files when a new type of route (page) is defined
 - [ ] Let the HTML to be the one that links its requirements (js and css)
- [ ] Media manager utility
- [ ] Let programmer set the default not found response
- [x] Manage basic defaults app and route headers
- [x] Manage route specific headers

## V2

- [ ] Study how to organize functions on structs or types if possible
- [ ] Allow middleware
    - [ ] Middleware to redirect or block petitions
    - [ ] Middleware to insert headers and data
- [ ] Log system
  - [ ] File management
  - [ ] Auto-cleanup
  - [ ] Transversal utility

***

## Improvements

- [ ] Check any potential error in Request Parsing and return a 500 error.
