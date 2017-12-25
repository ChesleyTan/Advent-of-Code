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

steps = 366

data FakeList = FakeList Int Int deriving (Show)

fakeInsert idx val (FakeList len x) = FakeList (len + 1) (if idx /= 1 then x else val)
fakeLength (FakeList len _) = len
fakeValue (FakeList _ val) = val

insertN :: Int -> (FakeList, Int) -> (FakeList, Int)
insertN !n (!xs, !currentPos) =
    let insertIdx = 1 + ((currentPos + steps) `mod` (fakeLength xs))
        newXs = fakeInsert insertIdx n xs
    in (newXs, insertIdx)

iterateN :: Int -> Int -> (FakeList, Int) -> (FakeList, Int)
--iterateN _ _ (xs, _) | trace (fakeValue xs |> show) False = undefined
iterateN !total !current (!xs, !currentPos)
    | current <= total = iterateN total (current + 1) (insertN current (xs, currentPos))
    | otherwise = (xs, currentPos)

main :: IO ()
main = do
    let (sequence, lastIdx) = iterateN 50000000 1 (FakeList 1 0, 0)
    print $ fakeValue $ sequence
