#[derive(Clone)]
pub struct Instruction {
    op: Box<dyn Op>,
    ret: usize,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{}: {}", self.ret, self.op)
    }
}

pub fn constant(value: f32, ret: usize) -> Instruction {
    Instruction {
        op: Box::new(ConstantOp {
            value,
        }),
        ret
    }
}

pub fn add(a: usize, b: usize, ret: usize) -> Instruction {
    Instruction {
        op: Box::new(AddOp {
            a,
            b,
        }),
        ret
    }
}

pub fn sub(a: usize, b: usize, ret: usize) -> Instruction {
    Instruction {
        op: Box::new(SubOp {
            a,
            b,
        }),
        ret
    }
}

pub fn mul(a: usize, b: usize, ret: usize) -> Instruction {
    Instruction {
        op: Box::new(MulOp {
            a,
            b,
        }),
        ret
    }
}

pub fn div(a: usize, b: usize, ret: usize) -> Instruction {
    Instruction {
        op: Box::new(DivOp {
            a,
            b,
        }),
        ret
    }
}

trait Op : std::fmt::Display {
    fn clone_box(&self) -> Box<dyn Op>;
}

impl Clone for Box<dyn Op> {
    fn clone(&self) -> Box<dyn Op> {
        self.clone_box()
    }
}

#[derive(Clone)]
struct ConstantOp {
    value: f32,
}

impl std::fmt::Display for ConstantOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "constant {}", self.value)
    }
}

impl Op for ConstantOp {
    fn clone_box(&self) -> Box<dyn Op> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
struct AddOp {
    a: usize,
    b: usize,
}

impl std::fmt::Display for AddOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "add %{} %{}", self.a, self.b)
    }
}

impl Op for AddOp {
    fn clone_box(&self) -> Box<dyn Op> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
struct SubOp {
    a: usize,
    b: usize,
}

impl std::fmt::Display for SubOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sub %{} %{}", self.a, self.b)
    }
}

impl Op for SubOp {
    fn clone_box(&self) -> Box<dyn Op> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
struct MulOp {
    a: usize,
    b: usize,
}

impl std::fmt::Display for MulOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mul %{} %{}", self.a, self.b)
    }
}

impl Op for MulOp {
    fn clone_box(&self) -> Box<dyn Op> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
struct DivOp {
    a: usize,
    b: usize,
}

impl std::fmt::Display for DivOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "div %{} %{}", self.a, self.b)
    }
}

impl Op for DivOp {
    fn clone_box(&self) -> Box<dyn Op> {
        Box::new(self.clone())
    }
}
