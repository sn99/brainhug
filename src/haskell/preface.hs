import Control.Monad
import Data.Char
import Data.Word

{-
This file was generated automatically from brainf*ck code

It uses a list zipper to represent the tape and monadic functions working with 'IO (Tape)' to represent elementary operations.
These functions are then combined via kleisli arrows to implement state-like behavior.
-}

type Cell = Word8
type Tape = ([Cell], [Cell])

left, right, inc, dec, putC, getC :: Tape -> IO Tape
left (xs, x:rs)     = return (x:xs, rs)
right (x:xs, rs)    = return (xs, x:rs) 
inc (x:xs, rs)      = return ((x+1):xs, rs)
dec (x:xs, rs)      = return ((x-1):xs, rs)
putC tape@(c:_,_)   = do putChar $ chr $ fromIntegral c
                         return tape
getC (_:xs,rs)      = do c <- (fromIntegral . ord) <$> getChar
                         return (c:xs, rs)

while :: (Tape -> IO Tape) -> Tape -> IO Tape
while _ tape@(0:_, _) = return tape 
while f tape          = do tape' <- f tape
                           while f tape'

main :: IO ()
main = void $ (flip ($)) (repeat 0, []) $ return