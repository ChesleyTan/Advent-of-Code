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

lastSndPlayedReg = "lastSndPlayed"

run :: M.Map Int Cmd -> Int -> M.Map String Int -> Int
run instructions pc regs
    | pc >= M.size instructions = error "No recovered freq!"
    | otherwise =
        case (M.findWithDefault (error "Invalid pc") pc instructions) of
            Snd a   -> run instructions (pc + 1) (M.insert lastSndPlayedReg (parseNum regs a) regs)
            Set a b -> run instructions (pc + 1) (M.insert a (parseNum regs b) regs)
            Add a b -> run instructions (pc + 1) (M.insert a (M.findWithDefault 0 a regs + parseNum regs b) regs)
            Mul a b -> run instructions (pc + 1) (M.insert a (M.findWithDefault 0 a regs * parseNum regs b) regs)
            Mod a b -> run instructions (pc + 1) (M.insert a (M.findWithDefault 0 a regs `mod` parseNum regs b) regs)
            Rcv a   -> if (parseNum regs a) /= 0
                        then M.findWithDefault 0 lastSndPlayedReg regs
                        else run instructions (pc + 1) regs
            Jgz a b -> if (parseNum regs a) > 0
                        then run instructions (pc + (parseNum regs b)) regs
                        else run instructions (pc + 1) regs

parseInstruction :: [String] -> Cmd
parseInstruction (cmd:args)
    | cmd == "snd" = Snd (head args)
    | cmd == "set" = Set (head args) (last args)
    | cmd == "add" = Add (head args) (last args)
    | cmd == "mul" = Mul (head args) (last args)
    | cmd == "mod" = Mod (head args) (last args)
    | cmd == "rcv" = Rcv (head args)
    | cmd == "jgz" = Jgz (head args) (last args)

main :: IO ()
main = do
    input <- map (map T.unpack . T.splitOn " ") . T.lines . T.strip <$> T.readFile "18a.input"
    let instructions = map parseInstruction input |> zip [0..] |> M.fromList
    print $ run instructions 0 M.empty
