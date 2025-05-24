package domain

const INVENTORY_ACCOUNT string = "10.01"

const INVENTORY_DIFF_ACCOUNT string = "10.02"

const RECEIVABLE_ACCOUNT string = "62.01"

const PAYABLE_ACCOUNT string = "60.01"

const SALES_ACCOUNT string = "90.01"

const COGS_ACCOUNT string = "90.02"

const DEBT string = "D"

const CREDIT string = "C"

type Account struct {
	Id   string
	Name string
}

func NewAccount(id, name string) Account {
	return Account{id, name}
}

func ChartOfAccounts() []Account {
	return []Account{
		NewAccount(INVENTORY_ACCOUNT, "Inventory"),
		NewAccount(INVENTORY_DIFF_ACCOUNT, "Inventory differences"),
		NewAccount(RECEIVABLE_ACCOUNT, "Account receivable"),
		NewAccount(PAYABLE_ACCOUNT, "Account payable"),
		NewAccount(SALES_ACCOUNT, "Sales"),
		NewAccount(COGS_ACCOUNT, "Cost of goods sold"),
	}
}
