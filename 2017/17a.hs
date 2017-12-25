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
import Numeric (showHex)
import Text.Printf (printf)

(|>) :: a -> (a -> b) -> b
(|>) = flip ($)

steps = 366

insertN :: Int -> ([Int], Int) -> ([Int], Int)
insertN n (xs, currentPos) =
    let insertIdx = 1 + ((currentPos + steps) `mod` (length xs))
        newXs = (take insertIdx xs) ++ [n] ++ (drop insertIdx xs)
    in (newXs, insertIdx)

iterateN :: Int -> Int -> ([Int], Int) -> ([Int], Int)
iterateN !total !current !(xs, currentPos)
    | current <= total = iterateN total (current + 1) (insertN current (xs, currentPos))
    | otherwise = (xs, currentPos)

main :: IO ()
main = do
    let (sequence, lastIdx) = iterateN 2017 1 ([0], 0)
    print $ sequence !! (lastIdx + 1)
