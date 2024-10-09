Stream-trade - Secondary marketplace for your vested contracts

Stream-trade marketplace program.

Marketplace Architecture
│
├── Marketplace Program
│   ├── Functions
│   │   ├── List Contract
│   │   ├── Buy Contract
│   │   └── Delist Contract
│   └── Error Handling
│       ├── Insufficient Funds
│       ├── Unauthorized Action
│       └── Already Sold
│
├── Accounts
│   ├── Marketplace Account
│   ├── Listing Account
│   │   ├── Seller (Pubkey)
│   │   ├── Price (u64)
│   │   ├── Stream Contract Address (Pubkey) - id
│   │   └── Status (Active/Sold/Delisted)
│   ├── Contract Account
│   ├── Seller Account
│   └── Buyer Account
│ 
├── Flow of Operations
│   ├── Listing Process 
│   │    └─ Seller calls list_contract 
│   │        └─ Creates Listing Account 
│   │        └─ Sets Price and Status 
│   ├── Buying Process 
│   │    └─ Buyer calls buy_contract 
│   │        └─ Checks Funds 
│   │        └─ Transfers Contract using Streamflow SDK 
│   │        └─ Transfers SOL to Seller 
│   │        └─ Updates Listing Status to Sold 
│  └── Delisting Process 
│      └─ Seller calls delist_contract 
│          └─ Checks Authorization 
│          └─ Updates Listing Status to Delisted 
│          
├── Streamflow SDK Integration 
│    ├── Transfer Struct 
│    │    ├── Authority (Seller) 
│    │    ├── New Recipient (Buyer) 
│    │    ├── New Recipient Tokens (Buyer’s Token Account) 
│    │    ├── Metadata (Contract Metadata) 
│    │    ├── Mint (Token Mint) 
│    │    └── Token Program (SPL Token Program)  
└── Frontend Interaction  
      ├── Querying Listings  
      │      └─ Call RPC Endpoint to Fetch Active Listings  
      └── Display Listings  
             └─ Show Price, Seller, and Status 



Targetted Streams to trade :- 

Any initial stream contracts that cannot be cancelled by neither sender nor receipient and can only be transferred by receipient are the targetted streams that the marketplace is aiming at.



