# Solana-bootcamp-final-projects

### Random Generative NFT
The NFT (Non-Fungible Tokens) produced by this smart contract with a random pattern can be minted, transferred, and burned in any order on Blockchain Solana.

#### Getting Started 

  - **Step 1: Build the contract**

    Open a terminal window from the terminal tab above and navigate to the generated directory using the command `cd program` (the folder name where you have Cargo.toml, file may be different) and type the command:
    <br>
    ```cargo build-sbf```           
  
  ---------------------------------
  - **Step 2: Set your config file**
  
    If there are no errors, type the command <br>
    ```solana config set --url devnet``` <br>
    This command will set our config file to connect to devnet, where we will deploy.
  
  ---------------------------------
  - **Step 3: Get devnet tokens**
  
    Deploying a contract will require some tokens, you get devnet tokens using the command <br>
    ```solana airdrop 1``` <br>
    You can get request as many airdrops as you need, after that you can check your balance with command <br>
    ```solana balance``` 
  
  ---------------------------------
  - **Step 4: Build and Deploy the Contract**
  
    Once we have the build, from the generated directory, type the following command for deployment: <br>
    ```solana program deploy target/deploy/nft.so``` 
    After completing the deployment, you will get a program ID.
  
    Copy and save this program ID so we can configure the client library.
  
  ---------------------------------
  - **Step 5: Download Dependencies**
  
    In order to run this code, we need to actually install these dependencies in our workspace. 
    In order to do that, open a terminal, navigate to the `program_client` directory (which contains `package.json`), and type the command `yarn install` to install the node_modules dependencies.
  
  ---------------------------------
  - **Step 6: Install SPL Token Library**
  
    This contract also relies on `@solana/spl-token`; this package needs to be installed manually by executing `yarn add @solana/spl-token`.
  
  ---------------------------------
  - **Step 7: Run app.ts**
  
    Now we have our frontend test as well as the contract, therefore we are ready to actually test our NFT contract. Execute the client by typing the following command:<br>
    `npx ts-node app.ts <YOUR_PROGRAM_ID>`
    

