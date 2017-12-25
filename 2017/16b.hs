{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BangPatterns #-}
import Control.Applicative ((<$>))
import Control.Monad (forM_)
import Data.Bits (xor, (.&.))
import Data.Char (ord)
import Data.Set as S (fromList, size, member, insert, empty)
import qualified Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member, foldr, foldrWithKey)
import Data.Maybe (fromJust)
import Data.Text as T (Text, strip, lines, words, unpack, splitOn)
import Data.Text.IO as T (readFile)
import Data.List (sortOn, group, foldl', elemIndex, iterate)
import Numeric (showHex)
import Text.Printf (printf)

(|>) :: a -> (a -> b) -> b
(|>) = flip ($)

update :: Int -> a -> [a] -> [a]
update !idx !newValue !xs = take idx xs ++ [newValue] ++ drop (idx + 1) xs

readInt :: String -> Int
readInt = read

type Line = String
exec :: String -> Line -> Line
exec "" !xs = xs
exec (cmd:args) !xs
    | cmd == 's' =
        let !amt = readInt args
            !rest = length xs - amt
        in drop rest xs ++ take rest xs
    | cmd == 'x' =
        let !idx1 = takeWhile (/= '/') args |> readInt
            !idx2 = dropWhile (/= '/') args |> drop 1 |> readInt
            !a = xs !! idx1
            !b = xs !! idx2
        in update idx1 b xs |> update idx2 a
    | cmd == 'p' =
        let !a = takeWhile (/= '/') args |> head
            !b = dropWhile (/= '/') args |> drop 1 |> head
            !idx1 = elemIndex a xs |> fromJust
            !idx2 = elemIndex b xs |> fromJust
        in update idx1 b xs |> update idx2 a
    | otherwise = error "Invalid cmd"

execAll :: [String] -> (M.Map Line Line, Line) -> (M.Map Line Line, Line)
execAll !input (!memo, !xs) =
    let memoResult = M.findWithDefault "" xs memo
    in if memoResult /= ""
        then (memo, memoResult)
        else
            let !newXs = foldr exec xs $ reverse input
                !newMemo = M.insert xs newXs memo
            in (newMemo, newXs)

iterateN :: Int -> Int -> (a -> a) -> a -> a
iterateN !total !current !f !a
    | current < total = iterateN total (current + 1) f (f a)
    | otherwise = a

-- Fun fact: The cycle is exactly 100
main :: IO String
main = do
    input <- map T.unpack . T.splitOn "," . T.strip <$> T.readFile "16a.input"
    let xs = ['a'..'p']
        --result = foldr (\_ acc -> execAll input acc) (M.empty, xs) [1..1000000000]
        result = iterateN 1000000000 0 (execAll input) (M.empty, xs)
    print $ result
    return ""
