mod log {
    enum Record {
        Account,
        Transaction,
    }

    // TODO: implement immutable log that receives either Account or Transaction Records. e.g:
    // Account::new(true, 100) -> Transaction::new("Burger King", 20, "2019-02-13T11:00:00.000Z") == Account(true, 80)
}
