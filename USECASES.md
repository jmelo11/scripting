# Lefi: Exploratory Use Cases

## Price Update Rule

In this example, we have a rule that will buy AAPL if the price is greater than 100.

```json
{
    "id": "uuid",
    "name": "AAPL Price Update",
    "description": "This rule will buy AAPL if the price is greater than 100",
    "script": [
        {
            "job": {
                "trigger": "PriceUpdate",
                "work": "UntilExecuted",
                "created_at": "2024-05-04",
                "status": "Active",
                "start_date": "2024-05-04",
                "end_date": "2024-05-04",
            },  
            "execution_policy": {
                "partial_fulfillment": "true",
                "cancel_unfulfilled": "false"
            },      
            "macros": [
                {
                    "key": "stock",
                    "value": "Spot(\"AAPL\")"
                },
                {
                    "key": "account",
                    "value": "StockAccount(\"1234-5678-9012-3456\")"
                },
                {
                    "key": "shares",
                    "value": "100"
                },
                {
                    "key": "executed",
                    "value": "false"
                }
            ],  
            "expression": "
                if stock > 100 then 
                    Sell(stock, account, shares);
                    executed = true;
                end
            "
        }
    ]
}    
```

Unresolved issues:

- What if the account does not have enough money to buy the shares?
- What if the account does not have enough shares to sell?

## Money Transfer Rule

```json
{
    "id": "uuid",
    "name": "Money Transfer",
    "description": "This rule will transfer money from one account to another if a condition is met",
    "script": [
        {
            "job": {
                "trigger": "TrasferUpdate",
                "work": "Monthly",
                "created_at": "2024-05-04",
                "status": "Active",
                "start_date": "2024-05-04",
                "end_date": "2024-05-04",
            },        
            "macros": [
                {
                    "key": "sender_account",
                    "value": "CashAccount(\"1234-5678-9012-3456\")"
                },
                {
                    "key": "receiver_account",
                    "value": "CashAccount(\"1234-5678-9012-3456\")"
                },
                {
                    "key": "amount",
                    "value": "100"
                }
            ],  
            "expression": "                
                Transfer(sender_account, receiver_account, amount);                
            "
        }
    ]
}    
```

Unresolved issues:

- What if the sender account does not have enough money to transfer?
- Who approves the transfer?
