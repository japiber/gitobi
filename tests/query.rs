use gitobi::query::{QryClause, QueryClause, QueryData};

#[test]
fn test_clause_eq() {
    let foo = "foo";
    let bar = "bar";
    let baz = "baz";
    let data = QueryData::new(&[(foo, bar)]);

    let c: QryClause = QueryClause::equal(foo, bar);
    assert_eq!(c.eval(&data), Ok(true));

    let c: QryClause = QueryClause::equal(foo, baz);
    assert_eq!(c.eval(&data), Ok(false));
}

#[test]
fn test_clause_neq() {
    let foo = "foo";
    let bar = "bar";
    let baz = "baz";
    let data = QueryData::new(&[(foo, bar)]);


    let c : QryClause = QueryClause::not_equal(foo, baz);
    assert_eq!(c.eval(&data), Ok(true));

    let c : QryClause = QueryClause::not_equal(foo, bar);
    assert_eq!(c.eval(&data), Ok(false));
}

#[test]
fn test_clause_ge() {
    let foo = "foo";
    let data = QueryData::new(&[(foo, 55)]);

    let c : QryClause = QueryClause::greater_or_equal_than(foo, 34);
    assert_eq!(c.eval(&data), Ok(true));

    let c : QryClause = QueryClause::greater_or_equal_than(foo, 55);
    assert_eq!(c.eval(&data), Ok(true));

    let c : QryClause = QueryClause::greater_or_equal_than(foo, 56);
    assert_eq!(c.eval(&data), Ok(false));
}

#[test]
fn test_clause_gt() {
    let foo = "foo";
    let data = QueryData::new(&[(foo, 55)]);

    let c : QryClause = QueryClause::greater_than(foo, 34);
    assert_eq!(c.eval(&data), Ok(true));

    let c : QryClause = QueryClause::greater_than(foo, 55);
    assert_eq!(c.eval(&data), Ok(false));

    let c : QryClause = QueryClause::greater_than(foo, 56);
    assert_eq!(c.eval(&data), Ok(false));
}

#[test]
fn test_clause_le() {
    let foo = "foo";
    let data = QueryData::new(&[(foo, 55)]);

    let c : QryClause = QueryClause::less_or_equal_than(foo, 56);
    assert_eq!(c.eval(&data), Ok(true));

    let c : QryClause = QueryClause::less_or_equal_than(foo, 55);
    assert_eq!(c.eval(&data), Ok(true));

    let c : QryClause = QueryClause::less_or_equal_than(foo, 54);
    assert_eq!(c.eval(&data), Ok(false));
}

#[test]
fn test_clause_lt() {
    let foo = "foo";
    let data = QueryData::new(&[(foo, 55)]);

    let c : QryClause = QueryClause::less_than(foo, 56);
    assert_eq!(c.eval(&data), Ok(true));

    let c : QryClause = QueryClause::less_than(foo, 55);
    assert_eq!(c.eval(&data), Ok(false));

    let c : QryClause = QueryClause::less_than(foo, 54);
    assert_eq!(c.eval(&data), Ok(false));
}

#[test]
fn test_clause_is_null() {
    let foo = "foo";
    let bar = "bar";
    let data = QueryData::new(&[(foo, None), (bar, Some(55))]);

    let c : QryClause = QueryClause::is_null(foo);
    assert_eq!(c.eval(&data), Ok(true));

    let c : QryClause = QueryClause::is_null(bar);
    assert_eq!(c.eval(&data), Ok(false));
}

#[test]
fn test_clause_and() {
    let foo = "foo";
    let bar = "bar";
    let data = QueryData::new(&[(foo, None), (bar, Some(55))]);

    let a : QryClause = QueryClause::and(
        QueryClause::is_null(foo),
        QueryClause::equal(bar, 55)
    );
    assert_eq!(a.eval(&data), Ok(true));

    let b : QryClause = QueryClause::and(
        QueryClause::is_null(foo),
        QueryClause::equal(bar, 5)
    );
    assert_eq!(b.eval(&data), Ok(false));
}

#[test]
fn test_clause_or() {
    let foo = "foo";
    let bar = "bar";
    let data = QueryData::new(&[(foo, None), (bar, Some(55))]);

    let a : QryClause = QueryClause::or(
        QueryClause::is_null(foo),
        QueryClause::equal(bar, 55)
    );
    assert_eq!(a.eval(&data), Ok(true));

    let b : QryClause = QueryClause::or(
        QueryClause::is_null(foo),
        QueryClause::equal(bar, 5)
    );
    assert_eq!(b.eval(&data), Ok(true));

    let c : QryClause = QueryClause::or(
        QueryClause::is_null(bar),
        QueryClause::equal(bar, 5)
    );
    assert_eq!(c.eval(&data), Ok(false));
}

#[test]
fn test_clause_not() {
    let foo = "foo";
    let bar = "bar";
    let data = QueryData::new(&[(foo, None), (bar, Some(55))]);

    let a : QryClause = QueryClause::not(
        QueryClause::equal(bar, 55)
    );
    assert_eq!(a.eval(&data), Ok(false));

    let b : QryClause = QueryClause::not(
        QueryClause::is_null(bar),
    );
    assert_eq!(b.eval(&data), Ok(true));
}
