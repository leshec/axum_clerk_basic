### Note to future self and anyone interested:

# 🚀 Getting Started with Axum + Clerk + Shuttle

### 🔐 Set up a Clerk account

1. Go to [Clerk.dev](https://clerk.dev) and create an account.
2. Create a new **application**.
3. Add a **dummy user** so you can test authentication.
4. Note we use a script tag in the index.html to access to JS Clerk SDK from a CDN. Alternatively, download it via NPM etc as per docs.

You’ll need two keys from Clerk:
- `CLERK_PUBLISHABLE_KEY` (frontend)
- `CLERK_SECRET_KEY` (backend)

### 🔑 Add your Clerk keys to the project

### You need to add two files and then include them in your .gitignore

1. **Create a `Secrets.toml` file** in the **root** of your project:

    ```toml
    CLERK_SECRET_KEY = "your-secret-key-here"
    ```

2. **Create a `.env` file** in the **root** of your project:

    ```env
    CLERK_PUBLISHABLE_KEY=your-publishable-key-here
    ```

> 🔎 Note: The `index.html` file includes a placeholder for the `CLERK_PUBLISHABLE_KEY` — make sure it matches your `.env` value.

### ⚙️ Install Shuttle

Make sure you have the Shuttle runtime installed. From your terminal, run:

```bash
curl -sSfL https://www.shuttle.dev/install | bash
```
For more shuttle install info go to [Shuttle.dev](https://docs.shuttle.dev/getting-started/installation)

```bash
git clone https://github.com/leshec/axum_clerk_basic.git
cd axum_clerk_basic
cargo update
shuttle run
```
Go to `http://127.0.0.1:8000/`


### Note: check the latest shuttle runtime and axum versions in the docs are right as that can be an issue.
Further note: Issue with out of date docs. To get Secrets.toml called in the main macro of axum.
Shuttle::secrets has been replaced with shuttle::runtime.


For production there are a whole load of other steps, see the Clerk docs. 

