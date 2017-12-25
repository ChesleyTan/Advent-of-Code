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

data Cmd = Set String String | Sub String String | Mul String String |
           Jnz String String deriving (Show)

readInt :: String -> Int
readInt = read

parseNum :: M.Map String Int -> String -> Int
parseNum regs a = M.findWithDefault (readInt a) a regs

mulCount :: String
mulCount = "mulCount"

run :: M.Map Int Cmd -> Int -> M.Map String Int -> Int
run instructions pc regs
    | pc >= M.size instructions = M.findWithDefault 0 mulCount regs
    | otherwise =
        case (M.findWithDefault (error "Invalid pc") pc instructions) of
            Set a b -> run instructions (pc + 1) (M.insert a (parseNum regs b) regs)
            Sub a b -> run instructions (pc + 1) (M.insert a (M.findWithDefault 0 a regs - parseNum regs b) regs)
            Mul a b ->
                let updatedMulCount = M.insert mulCount (M.findWithDefault 0 mulCount regs + 1) regs
                in run instructions (pc + 1) (M.insert a (M.findWithDefault 0 a updatedMulCount * parseNum updatedMulCount b) updatedMulCount)
            Jnz a b -> if (parseNum regs a) /= 0
                        then run instructions (pc + (parseNum regs b)) regs
                        else run instructions (pc + 1) regs

parseInstruction :: [String] -> Cmd
parseInstruction (cmd:args)
    | cmd == "set" = Set (head args) (last args)
    | cmd == "sub" = Sub (head args) (last args)
    | cmd == "mul" = Mul (head args) (last args)
    | cmd == "jnz" = Jnz (head args) (last args)

main :: IO ()
main = do
    input <- map (map T.unpack . T.splitOn " ") . T.lines . T.strip <$> T.readFile "23a.input"
    let instructions = map parseInstruction input |> zip [0..] |> M.fromList
        initRegs = M.fromList [("a", 0), ("b", 0), ("c", 0), ("d", 0), ("e", 0), ("f", 0), ("g", 0), ("h", 0)]
    print $ instructions
    print $ run instructions 0 initRegs
