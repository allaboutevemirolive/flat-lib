
use std::{fmt::Display, ops::Deref};

/// A type containing a trait object, which can have a blanket impl for types implementing that trait.
pub trait ParameterRef<'p, T> {
    fn from_param_ref(param: &'p T) -> Self;
}

/// A parameter for types implementing [Display]
pub struct DisplayParam<'a>(&'a dyn Display);

impl<'a> std::fmt::Debug for DisplayParam<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TestParam")
            .field(&self.0.to_string())
            .finish()
    }
}

impl<'a, T> ParameterRef<'a, T> for DisplayParam<'a>
where
    T: Display,
{
    fn from_param_ref(param: &'a T) -> Self {
        Self(param)
    }
}

pub trait Meta {
    type Param;
}

/// A sink for strings and parameters. Parameters can be borrowed.
pub trait Write<'param>: Meta + Sized
where
    <Self as Meta>::Param: 'param,
{
    fn param_ref<T: 'param>(&mut self, param: &'param T)
    where
        Self::Param: ParameterRef<'param, T>;

    fn write(&mut self, sql: impl AsRef<str>);

    fn expr<'e: 'param, E>(&mut self, expr: &'e E)
    where
        E: Expression<'param, Self> + 'e + ?Sized,
    {
        Expression::to_writer(expr, self)
    }
}

#[derive(Debug)]
pub struct TestWriter<Param> {
    sql: String,
    params: Vec<Param>,
}

impl<Param> Default for TestWriter<Param> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Param> TestWriter<Param> {
    pub fn new() -> Self {
        Self {
            sql: String::new(),
            params: Vec::new(),
        }
    }
}

impl<Param> Meta for TestWriter<Param> {
    type Param = Param;
}

impl<'param, Param: 'param> Write<'param> for TestWriter<Param> {
    fn param_ref<T: 'param>(&mut self, param: &'param T)
    where
        Self::Param: ParameterRef<'param, T>,
    {
        self.params.push(Self::Param::from_param_ref(param));
    }

    fn write(&mut self, sql: impl AsRef<str>) {
        self.sql.push_str(sql.as_ref())
    }
}

/// A type which can write to a writer, potentially borrowing parameters from itself to place in the writer.
pub trait Expression<'param, Writer>
where
    Writer: ?Sized,
{
    fn to_writer<'e: 'param>(&'e self, writer: &mut Writer)
    where
        Writer: Write<'param>,
        Writer::Param: 'param;

    fn boxed<'a>(self) -> Box<dyn Expression<'param, Writer> + 'a>
    where
        Self: Sized + 'a,
    {
        Box::new(self)
    }
}

impl<'param, T, Writer> Expression<'param, Writer> for Box<T>
where
    T: ?Sized + Expression<'param, Writer>,
    Writer: ?Sized,
{
    fn to_writer<'e: 'param>(&'e self, writer: &mut Writer)
    where
        Writer: Write<'param>,
        Writer::Param: 'param,
    {
        Box::deref(self).to_writer(writer)
    }
}

/// A sample expression which writes a static string and a reference to its inner parameter.
struct Test(usize);

impl<'param, W> Expression<'param, W> for Test
where
    W: Meta,
    W::Param: ParameterRef<'param, usize>,
{
    fn to_writer<'e: 'param>(&'e self, writer: &mut W)
    where
        W: Write<'param>,
        W::Param: 'param,
    {
        writer.write("TEST ");
        writer.write(self.0.to_string());
        writer.param_ref(&self.0);
    }
}

impl<'param, E, W, const N: usize> Expression<'param, W> for [E; N]
where
    E: Expression<'param, W>,
{
    fn to_writer<'e: 'param>(&'e self, writer: &mut W)
    where
        W: Write<'param>,
        W::Param: 'param,
    {
        let mut first = false;
        for e in self {
            if first {
                first = true;
            } else {
                writer.write(", ");
            }

            e.to_writer(writer)
        }
    }
}

fn main() {
    static_ty();
    // dyn_ty();
    // opaque_ty();
}

fn static_ty() {
    let mut writer = TestWriter::<DisplayParam>::new();

    let expr = [Test(1), Test(2), Test(3)];

    writer.expr(&expr);

    println!("{:?}", writer);
    // Output:
    // TestWriter { sql: ", TEST 1, TEST 2, TEST 3", params: [TestParam("1"), TestParam("2"), TestParam("3")] }
}

// fn dyn_ty() {
//     let mut writer = TestWriter::<DisplayParam>::new();

//     let expr = [Test(1).boxed(), Test(2).boxed(), Test(3).boxed()];

//     writer.expr(&expr);

//     println!("{:?}", writer);

//     drop(writer);
// }

// fn opaque_ty() {
//     fn opaque<'p, E, W>(e: E) -> impl Expression<'p, W>
//     where
//         E: Expression<'p, W>,
//         W: Write<'p>,
//         W::Param: 'p,
//     {
//         e
//     }

//     let mut writer = TestWriter::<DisplayParam>::new();

//     let expr = [opaque(Test(1)), opaque(Test(2)), opaque(Test(3))];

//     writer.expr(&expr);

//     println!("{:?}", writer);

//     drop(writer);
// }