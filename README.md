# Lefi: Scripting Language for Financial Automation

Scripting tools to automate tasks and money flows.

## Must have features

- [ ] ***Error Handling and Validation***
  - Add explicit error handling within the scripts to gracefully handle scenarios where insufficient funds or shares are available.
  - Implement informative error messages to let users know why a transaction failed.
- [ ] ***Approval Workflow***
  - Implement an approval workflow where certain transactions require manual or automated approval before execution.
  - Introduce role-based permissions to control who can approve transfers or investments.
- [ ] ***Notification System***
  - Send real-time notifications to clients for successful or failed transactions.
  - Allow users to customize alerts, such as via email or push notifications, for specific events (e.g., low balance).
- [ ] ***Pre-Transaction Checks***
  - Perform pre-transaction checks to ensure adequate balance in the sender's account or availability of shares before executing the transaction.
  - Notify the client if the conditions for execution aren't met.
- [ ] ***Audit Trail***
  - Maintain a detailed audit trail for every transaction to allow clients to see the history and reason for each action taken.
- [ ] ***Execution Policies***
  - Add execution policies that define whether to partially fulfill orders or cancel them altogether when conditions aren't met.
  - Implement user-configurable policies that determine how unfulfilled orders should be handled.
- [ ] ***Flexible Rule Management***
  - Enable clients to update or modify their own rules easily.
  - Consider version control for these rules to keep a record of changes.
- [ ] ***Rate Limits and Security***
  - Implement rate limits to prevent misuse of the API.
  - Secure sensitive information and ensure encryption during transactions.

## Language Methods

Methods can be categorized into the following groups:
    - **Information Retrieval**: Methods that fetch data from the system.
    - **Transaction Execution**: Methods that perform transactions.
    - **Notification and Logging**: Methods that send notifications or log messages.

The following methods are available in the scripting language:

- `Spot`: Get the current price of a stock or currency.
- `StockUnits`: Get the account information for a stock account.
- `PnL`: Get the profit and loss for position.
- `AccountBalance`: Get the account information for a cash account.
- `Sell`: Sell a stock or currency.
- `Buy`: Buy a stock or currency.
- `TransferAmount`: Transfer money between accounts.
- `Notify`: Send a notification to the user.
- `Print`: Print a message to the console.

## Special Lenguage Keywords

The following keywords are available in the scripting language:

- `authorize`: Request approval for a transaction.
- `finalize`: Flips the script's state if the scope is executed successfully.
- `execute`: Execute a transaction.
- `pays`: Indicates that the following value is a payment. Under the hood, it will be treated as a discounted value.
- `decide`: Execute a decision-making process.

### `authorize`

Request approval for a transaction.

- Authorizations are required for certain transactions, and this keyword initiates the approval process. This process is context-specific and may involve manual or automated approval.
- Technically, this keywords restricts the execution of the following statements until the transaction is approved.

***Example***

***Proposed syntax***

Implementing the `authorize` keyword as a scope block:

```lua
authorize {
    Buy("AAPL", "1234-5678-9012-3456", 100);
}
```

Interpretation: The `Buy` transaction will be executed only if the authorization is granted.

***Alternative syntax***

Implementing the `authorize` keyword as a function:

```lua
authorize = Authorize();
if authorize {
    Buy("AAPL", "1234-5678-9012-3456", 100);
}
```

### `finalize`

Flips the script's state if the scope is executed successfully.

- The `finalize` keyword is used to indicate that the script is in a valid state and can proceed to the next stage.

***Example***

```lua
finalize {
    Buy("AAPL", "1234-5678-9012-3456", 100);
}
```

Interpretation: If the `Buy` transaction is successful, the script will be marked as finalized and de-queued from the executor.

## Method Specifications

For each method, we provide the following information:

### `Spot`

---

`Spot` is a method that retrieves the current price of a stock or currency. Requires a pre-set provider to fetch the data.

- **Category**: Information Retrieval
- **Signature**: `Spot(symbol: str, provider: str) -> numeric`
- **Description**: Get the current (at eval0,
ation) price of a stock or currency.
- **Parameters**:
  - `symbol`: The symbol of the stock or currency (e.g., "AAPL" for Apple Inc.).
- **Returns**: The current price of the stock or currency.

***Example***

```lua
spot = Spot("AAPL", "YahooFinance");
if spot > 100 {
    Notify("AAPL price is above 100");
}
```

### `StockUnits`

---

This method retrieves the number of units of a stock held in a stock account. The account ID and stock symbol are required as parameters.

- **Category**: Information Retrieval
- **Signature**: `StockUnits(account_id: str, symbol: str) -> numeric`
- **Description**: Get the account information for a stock account.
- **Parameters**:
  - `account_id`: The unique identifier of the stock account.
  - `symbol`: The symbol of the stock (e.g., "AAPL" for Apple Inc.).
- **Returns**: The number of units of the stock held in the account.

***Example***

```lua
units = StockUnits("1234-5678-9012-3456", "AAPL"); 
if units < 100 {
    Notify("AAPL units are below 100"); 
} 
```

### `PnL`

---

Calculate the profit and loss for a position. The method requires the account ID and stock symbol as parameters. Can be used to determine stop-loss or take-profit levels.

- **Category**: Information Retrieval
- **Signature**: `PnL(account_id: str, symbol: str) -> numeric`
- **Description**: Get the profit and loss for a position.
- **Parameters**:
  - `account_id`: The unique identifier of the account.
  - `symbol`: The symbol of the stock (e.g., "AAPL" for Apple Inc.).
- **Returns**: The profit and loss for the position.
***Example***

```lua
pnl = PnL("1234-5678-9012-3456", "AAPL");
if pnl > 0 {
    Notify("AAPL position is profitable");
}
```

### `AccountBalance`

---

This method retrieves the current balance of a cash account. The balance represents the total amount of money available in the account. Accounts need to be pre-configured in the system.

- **Category**: Information Retrieval
- **Signature**: `AccountBalance(account_id: str) -> numeric`
- **Description**: Get the account information for a cash account.
- **Parameters**:
  - `account_id`: The unique identifier of the cash account.
- **Returns**: The current balance of the account.

***Example***

```lua
balance = AccountBalance("1234-5678-9012-3456");
Notify("Current balance is " + balance);
```

### `Sell`

---

- **Category**: Transaction Execution
- **Signature**: `Sell(symbol: str, account_id: str, units: numeric) -> bool`
- **Description**: Sell a stock or currency.
- **Parameters**:
  - `symbol`: The symbol of the stock or currency to sell.
  - `account_id`: The unique identifier of the account.
  - `units`: The number of units to sell.
- **Returns**: A boolean indicating whether the sale was successful.

***Example***

```lua
autorize {
  Sell("AAPL", "1234-5678-9012-3456", 100);
}
```

### `Buy`

---

`Buy` is a method that allows users to buy a stock or currency. The method requires the symbol of the stock or currency, the account ID, the number of units to buy and a provider to execute the transaction.

- **Category**: Transaction Execution
- **Signature**: `Buy(symbol: str, account_id: str, units: numeric) -> bool`
- **Description**: Buy a stock or currency.
- **Parameters**:
  - `symbol`: The symbol of the stock or currency to buy.
  - `account_id`: The unique identifier of the account.
  - `units`: The number of units to buy.
- **Returns**: A boolean indicating whether the purchase was successful.

***Example***

```lua
autorize {
  Buy("AAPL", "1234-5678-9012-3456", 100);
}
```

### `TransferAmount`

---

- **Category**: Transaction Execution
- **Signature**: `TransferAmount(sender_account_id: str, receiver_account_id: str, amount: numeric) -> bool`
- **Description**: Transfer money between accounts.
- **Parameters**:
  - `sender_account_id`: The unique identifier of the sender's account.
  - `receiver_account_id`: The unique identifier of the receiver's account.
  - `amount`: The amount of money to transfer.
- **Returns**: A boolean indicating whether the transfer was successful.

***Example***

```lua
autorize {
  TransferAmount("1234-5678-9012-3456", "5678-9012-3456-1234", 100);
}
```

### `Notify`

---

Notify the user with a message. Needs to be configured in the pre-stage. Notifications can be sent via email, SMS, or in-app notifications.

- **Category**: Notification and Logging
- **Signature**: `Notify(message: str) -> bool`
- **Description**: Send a notification to the user.
- **Parameters**:
  - `message`: The message to send as a notification.
- **Returns**: A boolean indicating whether the notification was sent successfully.
***Example***

```lua
Notify("AAPL price is above 100");
```
