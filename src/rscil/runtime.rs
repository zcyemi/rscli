#[allow(dead_code)]

pub enum Value{
    Int(i32),
    Float(f32),
    String(String),
    Char(char),
    Bool(bool),
}

pub struct StackMachine {

}


pub enum Instruction{
    Nop,
    Ldstr(String),
    Call(String),
    Ret,
    Ldarg(u8),
    Ldc(i32),
    Stloc(u8),
    Ldloc(u8),
    Add
}

pub struct CodeBlock {
    pub instructions:Vec<Instruction>
}

impl CodeBlock{
    pub fn new(ins:Vec<Instruction>)->CodeBlock{
        CodeBlock{instructions:ins}
    }

}


impl StackMachine {

    pub fn new()->StackMachine{
        StackMachine{}
    }

    pub fn exec(&self,codes:&CodeBlock)->i32{

        let mut stack:Vec<i32> = Vec::new();
        let mut heap:[i32;3] = [0;3];

        let x = codes.instructions.iter();

        let mut ret:i32 = 0;
        for ins in x  {
            match ins {
                Instruction::Ret =>{
                    ret = stack.pop().unwrap();
                    break;
                },
                Instruction::Ldc(v)=>{
                    stack.push(*v);
                },
                Instruction::Stloc(r)=>{
                    let v = stack.pop().unwrap();
                    heap[*r as usize] = v;
                },
                Instruction::Add=>{
                    let a = stack.pop().unwrap();
                    let b = stack.pop().unwrap();
                    stack.push(a+b);
                },
                Instruction::Ldloc(r)=>{
                    stack.push(heap[*r as usize]);
                }
                _ => {},
            }
        }
        ret
    }



    pub fn test(&self){
        let codeblock = CodeBlock::new(vec![
            Instruction::Ldc(10),
            Instruction::Stloc(0),
            Instruction::Ldc(20),
            Instruction::Stloc(1),
            Instruction::Ldloc(0),
            Instruction::Ldloc(1),
            Instruction::Add,
            Instruction::Ret
        ]);

        let ret= self.exec(&codeblock);
        println!("{:?}",ret);

    }
}