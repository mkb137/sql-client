/// SQL Type System constants.
pub(crate) enum TypeSystem {
    SqlServer2000 = 2000,
    SqlServer2005 = 2005,
    SqlServer2008 = 2008,
    SqlServer2012 = 2012,
}
pub(crate) struct TypeSystemVersion;
impl TypeSystemVersion {
    pub const LATEST: &'static str = "Latest";
    pub const SQL_SERVER_2000: &'static str = "SQL Server 2000";
    pub const SQL_SERVER_2005: &'static str = "SQL Server 2005";
    pub const SQL_SERVER_2008: &'static str = "SQL Server 2008";
    pub const SQL_SERVER_2012: &'static str = "SQL Server 2012";
}
