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
data Generator = Generator Factor State deriving (Show)

next :: Generator -> Generator
next (Generator factor state) =
    let nextVal = factor * state `mod` 2147483647
    in Generator factor nextVal

getState :: Generator -> State
getState (Generator _ state) = state

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
        aSeed   = 591
        bSeed   = 393
        aGen = Generator aFactor aSeed
        bGen = Generator bFactor bSeed
        updateCount (!aGen, !bGen, !count) = if checkMatchInt (getState aGen, getState bGen)
            then count + 1
            else count
        updateState (!aGen, !bGen, !count) =
            let newAGen = next aGen
                newBGen = next bGen
            in (newAGen, newBGen, updateCount (newAGen, newBGen, count))
    print $ foldr (const updateState) (aGen, bGen, 0) [0..40000000]
    return ""
