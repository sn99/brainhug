"""Python file compiled from Brainf*ck by brainhug [https://github.com/sn99/brainhug]"""
import sys

class InfiniteTape():
    TAPE_TAIL = 64
    
    def __init__(self, initial_capacity=64, zero_offset=32):
        self.tape = [0 for i in range(64)]
        self.zero_offset = zero_offset

    def _max_index(self):
        return len(self.tape) - self.zero_offset - 1
    
    def _min_index(self):
        return - self.zero_offset

    def _translate_index(self, index):
        """Convert a direct index into a 0-based index for self.tape"""
        return index + self.zero_offset

    def _check_index(self, index):
        _max = self._max_index()
        _min = self._min_index()
        _trg = self._translate_index(index)
        if _trg > _max:
            cells_to_add = max(_trg - _max + self.TAPE_TAIL, len(self.tape))
            self.tape += [0 for i in range(cells_to_add)]
        elif _trg < _min:
            cells_to_add = max(_min - _trg + self.TAPE_TAIL, len(self.tape))
            self.tape = [0 for i in range(cells_to_add)] + self.tape
            self.zero_offset += cells_to_add

    def __getitem__(self, index):
        """Access the cell contents at the given index"""
        self._check_index(index)
        return self.tape[self._translate_index(index)]

    def __setitem__(self, index, value):
        """Set the cell contents at the given index"""
        self._check_index(index)
        self.tape[self._translate_index(index)] = value

def read_char():
    return ord(sys.stdin.read(1))

def write_char(c):
    sys.stdout.write(chr(c))
    sys.stdout.flush()

def main():
    tape = InfiniteTape()
    index = 0 
