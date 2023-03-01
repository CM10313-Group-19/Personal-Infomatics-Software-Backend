## User authentication

### check_email
url: `/check_email`

type: `GET`

description: Checks if the email is in use

data: 
```json
{
    email: string //email address to check
}
```

response:
```json
{
    exists: bool // True if email is in use false if not
}
```

### signup
url: `/signup`

type: `POST`

description: Attempts to sign up a new user

data:
```json
{
    email: string, // The email of the new account
    password: string, // The password of the new account
    date_of_birth: string // The date of birth of the new account in YYYY-MM-DD format
}
```

response:
```json
{
    success: bool, // True if signup successfull False if not
    message: string, // If the signup fails the message why
}
```

### Login
url: `/login`

type: `GET`

description: Attempts to login a user

data:
```json
{
    email: string, // Email of the account
    password, string // Password of the account
}
```

response:
```json
{
    success: bool, // True if login was a success False if not
    user_id: int // If the login was successfull the user id, else -1
}
```

