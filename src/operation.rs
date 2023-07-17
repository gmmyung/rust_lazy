use std::{cell::RefCell, fmt::Display, rc::Rc};

mod instruction;

#[derive(Clone)]
pub struct Scalar<O: Operation> {
    operation: Rc<RefCell<O>>,
}

impl Scalar<Constant> {
    pub fn new(value: f32) -> Self {
        Self {
            operation: Rc::new(RefCell::new(Constant {
                value,
                compile_ret: None,
            })),
        }
    }
}

impl<O: Operation> Scalar<O> {
    pub fn execute(&self) -> f32 {
        self.operation.borrow().execute()
    }

    pub fn compile(self) -> Vec<instruction::Instruction> 
    {
        let mut operand_num_iterator = (0..).into_iter();
        match self.operation.borrow_mut().compile(&mut operand_num_iterator) {
            CompileResult::AlreadyCompiled(_) => unreachable!(),
            CompileResult::Compiled(instructions, _) => instructions,
        }
    }
}

impl<O: Operation> Display for Scalar<O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.operation.borrow().to_string())
    }
}

pub enum CompileResult {
    AlreadyCompiled(usize),
    Compiled(Vec<instruction::Instruction>, usize),
}

impl CompileResult {
    fn get_ret(&self) -> usize {
        match self {
            CompileResult::AlreadyCompiled(ret) => *ret,
            CompileResult::Compiled(_, ret) => *ret,
        }
    }

    fn get_instructions(&self) -> Option<Vec<instruction::Instruction>> {
        match self {
            CompileResult::AlreadyCompiled(_) => None,
            CompileResult::Compiled(instructions, _) => Some(instructions.clone()),
        }
    }
}

impl Display for CompileResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileResult::AlreadyCompiled(_) => write!(f, "Compiled Already"),
            CompileResult::Compiled(instructions, ret) => {
                let mut s = String::new();
                for instruction in instructions {
                    s.push_str(&format!("{}\n", instruction));
                }
                write!(f, "{}ret {}", s, ret)
            }
        }
    }
}

pub trait Operation: Display + Clone {
    fn execute(&self) -> f32;
    fn compile<I>(&mut self, operand_num_iterator: &mut I) -> CompileResult
    where
        I: Iterator<Item = usize>;
}

#[derive(Clone)]
pub struct Constant {
    value: f32,
    compile_ret: Option<usize>,
}

impl Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Operation for Constant {
    fn execute(&self) -> f32 {
        self.value
    }

    fn compile<I>(&mut self, operand_num_iterator: &mut I) -> CompileResult
    where
        I: Iterator<Item = usize>,
    {
        match self.compile_ret {
            Some(ret) => CompileResult::AlreadyCompiled(ret),
            None => {
                let ret = operand_num_iterator.next().unwrap();
                self.compile_ret = Some(ret);
                CompileResult::Compiled(vec![instruction::constant(self.value, ret)], ret)
            }
        }
    }
}

#[derive(Clone)]
pub struct Add<T: Operation, U: Operation> {
    a: Scalar<T>,
    b: Scalar<U>,
    compile_ret: Option<usize>,
}

impl<T, U> Add<T, U>
where
    T: Operation,
    U: Operation,
{
    fn new(a: &Scalar<T>, b: &Scalar<U>) -> Self {
        Self {
            a: a.clone(),
            b: b.clone(),
            compile_ret: None,
        }
    }
}

impl<O: Operation> Scalar<O> {
    pub fn add<U>(&self, other: &Scalar<U>) -> Scalar<Add<O, U>>
    where
        U: Operation,
    {
        Scalar {
            operation: Rc::new(RefCell::new(Add::new(self, other))),
        }
    }
}

impl<T, U> std::ops::Add<&Scalar<U>> for &Scalar<T>
where
    T: Operation,
    U: Operation,
{
    type Output = Scalar<Add<T, U>>;

    fn add(self, other: &Scalar<U>) -> Self::Output {
        self.add::<U>(other)
    }
}

impl<T, U> Display for Add<T, U>
where
    T: Operation,
    U: Operation,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} + {})", self.a, self.b)
    }
}

impl<T, U> Operation for Add<T, U>
where
    T: Operation,
    U: Operation,
{
    fn execute(&self) -> f32 {
        self.a.operation.borrow().execute() + self.b.operation.borrow().execute()
    }

    fn compile<I>(&mut self, operand_num_iterator: &mut I) -> CompileResult
    where
        I: Iterator<Item = usize>,
    {
        match self.compile_ret {
            Some(ret) => CompileResult::AlreadyCompiled(ret),
            None => {
                let a = self.a.operation.borrow_mut().compile(operand_num_iterator);
                let b = self.b.operation.borrow_mut().compile(operand_num_iterator);
                let mut instructions = Vec::new();
                a.get_instructions().map(|i| instructions.extend(i));
                b.get_instructions().map(|i| instructions.extend(i));
                let ret = operand_num_iterator.next().unwrap();
                self.compile_ret = Some(ret);
                instructions.push(instruction::add(a.get_ret(), b.get_ret(), ret));
                CompileResult::Compiled(instructions, ret)
            }
        }
    }
}

#[derive(Clone)]
pub struct Sub<T: Operation, U: Operation> {
    a: Scalar<T>,
    b: Scalar<U>,
    compile_ret: Option<usize>,
}

impl<T, U> Sub<T, U>
where
    T: Operation,
    U: Operation,
{
    fn new(a: &Scalar<T>, b: &Scalar<U>) -> Self {
        Self {
            a: a.clone(),
            b: b.clone(),
            compile_ret: None,
        }
    }
}

impl<O: Operation> Scalar<O> {
    pub fn sub<U>(&self, other: &Scalar<U>) -> Scalar<Sub<O, U>>
    where
        U: Operation,
    {
        Scalar {
            operation: Rc::new(RefCell::new(Sub::new(self, other))),
        }
    }
}

impl<T, U> std::ops::Sub<&Scalar<U>> for &Scalar<T>
where
    T: Operation,
    U: Operation,
{
    type Output = Scalar<Sub<T, U>>;

    fn sub(self, other: &Scalar<U>) -> Self::Output {
        self.sub::<U>(other)
    }
}

impl<T, U> Display for Sub<T, U>
where
    T: Operation,
    U: Operation,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} - {})", self.a, self.b)
    }
}

impl<T, U> Operation for Sub<T, U>
where
    T: Operation,
    U: Operation,
{
    fn execute(&self) -> f32 {
        self.a.operation.borrow().execute() - self.b.operation.borrow().execute()
    }

    fn compile<I>(&mut self, operand_num_iterator: &mut I) -> CompileResult
    where
        I: Iterator<Item = usize>,
    {
        match self.compile_ret {
            Some(ret) => CompileResult::AlreadyCompiled(ret),
            None => {
                let a = self.a.operation.borrow_mut().compile(operand_num_iterator);
                let b = self.b.operation.borrow_mut().compile(operand_num_iterator);
                let mut instructions = Vec::new();
                a.get_instructions().map(|i| instructions.extend(i));
                b.get_instructions().map(|i| instructions.extend(i));
                let ret = operand_num_iterator.next().unwrap();
                self.compile_ret = Some(ret);
                instructions.push(instruction::sub(a.get_ret(), b.get_ret(), ret));
                CompileResult::Compiled(instructions, ret)
            }
        }
    }
}

#[derive(Clone)]
pub struct Mul<T: Operation, U: Operation> {
    a: Scalar<T>,
    b: Scalar<U>,
    compile_ret: Option<usize>,
}

impl<T, U> Mul<T, U>
where
    T: Operation,
    U: Operation,
{
    fn new(a: &Scalar<T>, b: &Scalar<U>) -> Self {
        Self {
            a: a.clone(),
            b: b.clone(),
            compile_ret: None,
        }
    }
}

impl<O: Operation> Scalar<O> {
    pub fn mul<U>(&self, other: &Scalar<U>) -> Scalar<Mul<O, U>>
    where
        U: Operation,
    {
        Scalar {
            operation: Rc::new(RefCell::new(Mul::new(self, other))),
        }
    }
}

impl<T, U> std::ops::Mul<&Scalar<U>> for &Scalar<T>
where
    T: Operation,
    U: Operation,
{
    type Output = Scalar<Mul<T, U>>;

    fn mul(self, other: &Scalar<U>) -> Self::Output {
        self.mul::<U>(other)
    }
}

impl<T, U> Display for Mul<T, U>
where
    T: Operation,
    U: Operation,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} * {})", self.a, self.b)
    }
}

impl<T, U> Operation for Mul<T, U>
where
    T: Operation,
    U: Operation,
{
    fn execute(&self) -> f32 {
        self.a.operation.borrow().execute() * self.b.operation.borrow().execute()
    }

    fn compile<I>(&mut self, operand_num_iterator: &mut I) -> CompileResult
    where
        I: Iterator<Item = usize>,
    {
        match self.compile_ret {
            Some(ret) => CompileResult::AlreadyCompiled(ret),
            None => {
                let a = self.a.operation.borrow_mut().compile(operand_num_iterator);
                let b = self.b.operation.borrow_mut().compile(operand_num_iterator);
                let mut instructions = Vec::new();
                a.get_instructions().map(|i| instructions.extend(i));
                b.get_instructions().map(|i| instructions.extend(i));
                let ret = operand_num_iterator.next().unwrap();
                self.compile_ret = Some(ret);
                instructions.push(instruction::mul(a.get_ret(), b.get_ret(), ret));
                CompileResult::Compiled(instructions, ret)
            }
        }
    }
}

#[derive(Clone)]
pub struct Div<T: Operation, U: Operation> {
    a: Scalar<T>,
    b: Scalar<U>,
    compile_ret: Option<usize>,
}

impl<T, U> Div<T, U>
where
    T: Operation,
    U: Operation,
{
    fn new(a: &Scalar<T>, b: &Scalar<U>) -> Self {
        Self {
            a: a.clone(),
            b: b.clone(),
            compile_ret: None,
        }
    }
}

impl<O: Operation> Scalar<O> {
    pub fn div<U>(&self, other: &Scalar<U>) -> Scalar<Div<O, U>>
    where
        U: Operation,
    {
        Scalar {
            operation: Rc::new(RefCell::new(Div::new(self, other))),
        }
    }
}

impl<T, U> std::ops::Div<&Scalar<U>> for &Scalar<T>
where
    T: Operation,
    U: Operation,
{
    type Output = Scalar<Div<T, U>>;

    fn div(self, other: &Scalar<U>) -> Self::Output {
        self.div::<U>(other)
    }
}

impl<T, U> Display for Div<T, U>
where
    T: Operation,
    U: Operation,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} / {})", self.a, self.b)
    }
}

impl<T, U> Operation for Div<T, U>
where
    T: Operation,
    U: Operation,
{
    fn execute(&self) -> f32 {
        self.a.operation.borrow().execute() / self.b.operation.borrow().execute()
    }

    fn compile<I>(&mut self, operand_num_iterator: &mut I) -> CompileResult
    where
        I: Iterator<Item = usize>,
    {
        match self.compile_ret {
            Some(ret) => CompileResult::AlreadyCompiled(ret),
            None => {
                let a = self.a.operation.borrow_mut().compile(operand_num_iterator);
                let b = self.b.operation.borrow_mut().compile(operand_num_iterator);
                let mut instructions = Vec::new();
                a.get_instructions().map(|i| instructions.extend(i));
                b.get_instructions().map(|i| instructions.extend(i));
                let ret = operand_num_iterator.next().unwrap();
                self.compile_ret = Some(ret);
                instructions.push(instruction::div(a.get_ret(), b.get_ret(), ret));
                CompileResult::Compiled(instructions, ret)
            }
        }
    }
}
