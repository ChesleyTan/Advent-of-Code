{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BangPatterns #-}
import Control.Applicative ((<$>))
import Control.Monad (forM_)
import Data.Bits (xor, (.&.))
import Data.Char (ord)
import Data.Set as S (fromList, size, member, insert, empty)
import Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member, foldr, foldrWithKey)
import Data.Text as T (Text, strip, lines, words, unpack, splitOn)
import Data.Text.IO as T (readFile)
import Data.List (sortOn, group, foldl', elemIndex, iterate)
import Debug.Trace (trace)
import Numeric (showHex)
import Text.Printf (printf)

(|>) :: a -> (a -> b) -> b
(|>) = flip ($)

data Cmd = Snd String | Set String String | Add String String | Mul String String
           | Mod String String | Rcv String | Jgz String String deriving (Show)

readInt :: String -> Int
readInt = read

parseNum :: M.Map String Int -> String -> Int
parseNum regs a = M.findWithDefault (readInt a) a regs

numSends :: String
numSends = "numSends"

type ProgramCounter = Int
type NetworkBuffer = [Int]
type SendBuffer = NetworkBuffer
type ReceiveBuffer = NetworkBuffer
type Registers = M.Map String Int
type Instructions = M.Map Int Cmd

evalSingle :: Cmd -> ProgramCounter -> Registers -> ReceiveBuffer -> SendBuffer -> (ProgramCounter, Registers, ReceiveBuffer, SendBuffer)
evalSingle cmd pc regs rcvBuffer sndBuffer = case cmd of
    Snd a   -> (pc + 1, M.insert numSends (M.findWithDefault 0 numSends regs + 1) regs, rcvBuffer, sndBuffer ++ [parseNum regs a])
    Set a b -> (pc + 1, M.insert a (parseNum regs b) regs, rcvBuffer, sndBuffer)
    Add a b -> (pc + 1, M.insert a (M.findWithDefault 0 a regs + parseNum regs b) regs, rcvBuffer, sndBuffer)
    Mul a b -> (pc + 1, M.insert a (M.findWithDefault 0 a regs * parseNum regs b) regs, rcvBuffer, sndBuffer)
    Mod a b -> (pc + 1, M.insert a (M.findWithDefault 0 a regs `mod` parseNum regs b) regs, rcvBuffer, sndBuffer)
    Rcv a   -> if not (null rcvBuffer)
                then (pc + 1, M.insert a (head rcvBuffer) regs, tail rcvBuffer, sndBuffer)
                else (pc, regs, rcvBuffer, sndBuffer) -- no-op
    Jgz a b -> if parseNum regs a > 0
                then (pc + parseNum regs b, regs, rcvBuffer, sndBuffer)
                else (pc + 1, regs, rcvBuffer, sndBuffer)

run :: Instructions -> ProgramCounter -> ProgramCounter -> Registers -> Registers -> NetworkBuffer -> NetworkBuffer -> Int
run instructions pc0 pc1 regs0 regs1 buffer0 buffer1
    | pc0 >= M.size instructions || pc1 >= M.size instructions = M.findWithDefault 0 numSends regs1
    | otherwise =
        let cmd0 = M.findWithDefault (error "Invalid pc") pc0 instructions
            cmd1 = M.findWithDefault (error "Invalid pc") pc1 instructions
            (newPc0, newRegs0, tmpBuffer0, tmpBuffer1) = evalSingle cmd0 pc0 regs0 buffer0 buffer1
            (newPc1, newRegs1, newBuffer1, newBuffer0) = evalSingle cmd1 pc1 regs1 tmpBuffer1 tmpBuffer0
        in case (cmd0, cmd1) of
            (Rcv _, Rcv _) | newPc0 == pc0 && newPc1 == pc1 -> M.findWithDefault 0 numSends newRegs1
            _ -> run instructions newPc0 newPc1 newRegs0 newRegs1 newBuffer0 newBuffer1

parseInstruction :: [String] -> Cmd
parseInstruction [] = error "invalid instruction"
parseInstruction (cmd:args)
    | cmd == "snd" = Snd (head args)
    | cmd == "set" = Set (head args) (last args)
    | cmd == "add" = Add (head args) (last args)
    | cmd == "mul" = Mul (head args) (last args)
    | cmd == "mod" = Mod (head args) (last args)
    | cmd == "rcv" = Rcv (head args)
    | cmd == "jgz" = Jgz (head args) (last args)
    | otherwise = error "invalid instruction"

main :: IO ()
main = do
    input <- map (map T.unpack . T.splitOn " ") . T.lines . T.strip <$> T.readFile "18a.input"
    let instructions = map parseInstruction input |> zip [0..] |> M.fromList
    print $ run instructions 0 0 (M.fromList [("p", 0)]) (M.fromList [("p", 1)]) [] []
