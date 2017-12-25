{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BangPatterns #-}
import Control.Monad (forM_)
import Data.Bits (xor, (.&.))
import Data.Char (ord)
import Data.Set as S (fromList, size, member, insert, empty)
import qualified Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member, foldr, foldrWithKey)
import Data.Text as T (Text, strip, lines, words, unpack, splitOn)
import Data.Text.IO as T (readFile)
import Control.Applicative ((<$>))
import Data.List (sortOn, group, foldl')
import Numeric (showHex)
import Text.Printf (printf)

(|>) = flip ($)

type Factor = Int
type State = Int
type Multiple = Int
data Generator = Generator Factor Multiple State deriving (Show)

next :: Generator -> Generator
next (Generator factor multiple state) =
    let nextVal = factor * state `mod` 2147483647
    in if nextVal `mod` multiple == 0
        then Generator factor multiple nextVal
        else next (Generator factor multiple nextVal)

getState :: Generator -> State
getState (Generator _ _ state) = state

toBin :: Int -> String -> String
toBin !x !acc
    | x > 0 = toBin (x `div` 2) (show (x `mod` 2) ++ acc)
    | otherwise = acc

pad :: Int -> String -> String
pad !len !s
    | length s < len = pad len ('0':s)
    | otherwise = s

toBin32 :: Int -> String
toBin32 !x = toBin x "" |> pad 32

checkMatch :: (String, String) -> Bool
checkMatch (!a, !b) = (reverse a |> take 16) == (reverse b |> take 16)

checkMatchInt :: (Int, Int) -> Bool
checkMatchInt (a, b) = a .&. 65535 == b .&. 65535

main :: IO String
main = do
    let aFactor = 16807
        bFactor = 48271
        aMultiple = 4
        bMultiple = 8
        aSeed   = 591
        bSeed   = 393
        aGen = Generator aFactor aMultiple aSeed
        bGen = Generator bFactor bMultiple bSeed
        -- Could clean this up using `iterate`
        updateCount (!aGen, !bGen, !count) = if checkMatchInt (getState aGen, getState bGen)
            then count + 1
            else count
        updateState (!aGen, !bGen, !count) =
            let newAGen = next aGen
                newBGen = next bGen
            in (newAGen, newBGen, updateCount (newAGen, newBGen, count))
    print $ foldr (const updateState) (aGen, bGen, 0) [0..5000000]
    return ""
