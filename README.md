# Solana CRUD App

A decentralized journal application built on Solana using the Anchor framework. This application allows users to create, read, update, and delete journal entries stored on the Solana blockchain.

## Project Structure

```
├── programs/crud-app/          # Solana program (smart contract)
│   └── src/lib.rs             # Main program logic
├── tests/                     # Test files
│   └── crud-app.ts           # Program tests
├── migrations/                # Deployment scripts
│   └── deploy.ts             # Deploy script
├── target/                   # Build artifacts
│   ├── idl/crud_app.json    # Generated IDL
│   └── types/               # TypeScript types
├── Anchor.toml              # Anchor configuration
├── Cargo.toml              # Rust dependencies
└── package.json            # Node.js dependencies
```

## Features

- **Create Journal Entries**: Users can create journal entries with a title and description
- **Decentralized Storage**: All data is stored on the Solana blockchain
- **User Ownership**: Each journal entry is owned by the creator's wallet
- **Unique Identification**: Entries are identified using Program Derived Addresses (PDAs)

## Program Details

The smart contract ([programs/crud-app/src/lib.rs](programs/crud-app/src/lib.rs)) includes:

- **`initialize`** function: Creates new journal entries
- **`JournalState`** struct: Defines the data structure for journal entries
  - `owner`: Public key of the journal owner
  - `title`: Journal title (max 50 characters)
  - `description`: Journal description (max 1000 characters)
- **`GetJournalEntry`** context: Handles account validation and PDA creation

## Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor Framework](https://www.anchor-lang.com/docs/installation)
- [Node.js](https://nodejs.org/) (v16 or higher)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd crud-app
```

2. Install dependencies:
```bash
npm install
```

3. Build the program:
```bash
anchor build
```

## Configuration

The project is configured in [Anchor.toml](Anchor.toml). Make sure your Solana CLI is configured to use the correct network:

```bash
solana config set --url localhost  # For local development
# or
solana config set --url devnet     # For devnet testing
```

## Usage

### Deploy the Program

1. Start a local Solana validator (for local testing):
```bash
solana-test-validator
```

2. Deploy the program:
```bash
anchor deploy
```

### Run Tests

Execute the test suite in [tests/crud-app.ts](tests/crud-app.ts): ( Test not written Yet)

```bash
anchor test
```

### Interact with the Program

The program can be interacted with using the generated TypeScript client or through other Solana tools. The program accepts:

- `title`: String (max 50 characters)
- `description`: String (max 1000 characters)

## Program ID

The program is deployed with ID: `JCbdvjeep12RTG6B1N8XmQ8dxuoGk4PJAK5ioqUjBU2C`

## Development

### Adding New Features

1. Modify the program logic in [programs/crud-app/src/lib.rs](programs/crud-app/src/lib.rs)
2. Update tests in [tests/crud-app.ts](tests/crud-app.ts)
3. Rebuild and test:
```bash
anchor build
anchor test
```

### Account Structure

Journal entries are stored using Program Derived Addresses (PDAs) with seeds:
- `title.as_bytes()`
- `owner.key().as_ref()`

This ensures each user can have unique journal entries identified by title.

## Testing

The test file ([tests/crud-app.ts](tests/crud-app.ts)) demonstrates:
- Program initialization
- Creating journal entries
- Transaction signing and execution

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request


## Resources

- [Anchor Documentation](https://www.anchor-lang.com/)
- [Solana Documentation](https://docs.solana.com/)
- [Solana Cookbook](https://solanacookbook.com/)