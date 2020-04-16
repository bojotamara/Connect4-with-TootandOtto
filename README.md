# Connect4 with Toot and Otto

### Step 1: Install MongoDB
#### Windows
1. Download the MongoDB installer file from the downloads section of the MongoDB website.
2. Find the dowloaded .msi file in the Windows Explorer. Double click the file and follow the prompts to install Mongo.
3. Start the mongodb daemon by running "C:\Program Files\MongoDB\Server\4.2\bin\mongo.exe" in the Command Prompt.

#### MacOS with Homebrew
1. Open the Terminal app and type

    ```
    $ brew update.
    ```

2. After updating Homebrew

    ```
    $ install mongodb
    ```

3. Start the Mongo server by typing:

    ```
    $ mongod
    ```

### Step 2: Starting the backend

1. Open a terminal and navigate to the `backend` folder.

2. Run the backend on Rust's nightly distribution:

    ```
    cargo +nightly run
    ```

3. The base backend routes can be found at http://localhost:8000.  This does not need to be open to run the game website, but the backend does need to be running in a terminal.

### Step 3: Build and run the frontend

1. To build the project, open the terminal in the root of the project and run:

    ```
    wasm-pack build
    ```

2. cd into the `www` folder and run:

    ```
    npm install
    ```

3. Once it has finished installing the dependencies, run:

    ```
    npm start
    ```

4. Visit http://localhost:8080

### Step 4: Play the games!

1. You should now be at the homepage for the game website. Click on the sidebar links to access instructions for Connect4/Toot and Otto, or jump straight into playing!
