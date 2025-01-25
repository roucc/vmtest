class Frame():
    def __init__(self,code):
        self.stack = []
        self.instructions = []
        self.pc = 0
        self.code = code
        self.retstack = []

        self.labels = {}

    def push(self, a):
        self.stack.append(int(a))

    def pop(self):
        if not self.stack:
            raise IndexError("stack empty")
        return self.stack.pop()

    def inc(self):
        self.push(self.pop() + 1)
    
    def dec(self):
        self.push(self.pop() - 1)

    def jump(self, label):
        # Jump to the instruction with the given label
        self.pc = self.labels[label]

    def jnz(self, label):
        # jump if not zero
        condition = self.pop()
        self.push(condition)
        if condition != 0:
            self.pc = self.labels[label] -1

    def label(self, name):
        pass

    def dup(self): # ( a -- a a )
        a=self.pop()
        self.push(a)
        self.push(a)
    
    def swap(self): # ( a b -- b a )
        a=self.pop()
        b=self.pop()
        self.push(a)
        self.push(b)
    
    def over(self): # ( a b -- a b a )
        #duplicate second down
        self.push(self.stack[-2])
    
    def pick(self, i):  # ( -- v )
        self.push(self.stack[-int(i)])
    
    def poke(self, i): # ( v -- )
        self.stack[-int(i)]=self.pop()

    def call(self,label):
        pc = self.labels[label]
        self.retstack.append(self.pc)
        self.pc = pc

    def ret(self):
        self.pc = self.retstack.pop()

    def end(self):
        # Halt the program
        self.pc = len(self.instructions)
        print("program halted")

    # optimal (two pass):
    # first pass should load all labels with pc
    # second pass runs but can just look up addresses in dict.
    def find_labels(self):
        for pc, instructions in enumerate(self.instructions):
            if instructions[0] == "label":
                self.labels[instructions[1]] = pc
    
    def print(self):
        print(f"current stack: {self.stack}")
        print(f"pc: {self.pc}")
        print(f"current instruction: {self.instructions[self.pc]}")
        print()
    
    def compile(self):
        for line in self.code.strip().split("\n"):
            line=line.strip()
            if line == "":
                continue
            try:
                self.instructions.append(tuple(line.strip().split()))
            except ValueError:
                self.instructions.append((line.strip()))
            
    def run(self):
        self.compile()
        self.find_labels()
        while self.pc < len(self.instructions):
            self.print()
            instruction = self.instructions[self.pc]
            opcode = instruction[0]
            operand = instruction[1:]
            getattr(self, opcode)(*operand)
            self.pc += 1

instructions = """
    push 8
    push 4
    call mul
    end

    label mul
    push 0
    push 0
        label loop
            pop
            over
            call add
            pick 3
            dec
            dup
            poke 4
            jnz loop
        pop
        poke 2
        pop
        ret

    label add
        swap
        inc
        swap
        dec
        jnz add
        pop
        ret
"""

f = Frame(instructions)
f.run()
