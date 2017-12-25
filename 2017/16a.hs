{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BangPatterns #-}
import Control.Monad (forM_)
import Data.Bits (xor, (.&.))
import Data.Char (ord)
import Data.Set as S (fromList, size, member, insert, empty)
import qualified Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member, foldr, foldrWithKey)
import Data.Maybe (fromJust)
import Data.Text as T (Text, strip, lines, words, unpack, splitOn)
import Data.Text.IO as T (readFile)
import Control.Applicative ((<$>))
import Data.List (sortOn, group, foldl', elemIndex)
import Numeric (showHex)
import Text.Printf (printf)

(|>) :: a -> (a -> b) -> b
(|>) = flip ($)

update :: Int -> a -> [a] -> [a]
update idx newValue xs = take idx xs ++ [newValue] ++ drop (idx + 1) xs

readInt :: String -> Int
readInt = read

type Line = String
exec :: String -> Line -> Line
exec "" xs = xs
exec (cmd:args) xs
    | cmd == 's' =
        let amt = readInt args
            rest = length xs - amt
        in drop rest xs ++ take rest xs
    | cmd == 'x' =
        let idx1 = takeWhile (/= '/') args |> readInt
            idx2 = dropWhile (/= '/') args |> drop 1 |> readInt
            a = xs !! idx1
            b = xs !! idx2
        in update idx1 b xs |> update idx2 a
    | cmd == 'p' =
        let a = takeWhile (/= '/') args |> head
            b = dropWhile (/= '/') args |> drop 1 |> head
            idx1 = elemIndex a xs |> fromJust
            idx2 = elemIndex b xs |> fromJust
        in update idx1 b xs |> update idx2 a
    | otherwise = error "Invalid cmd"

main :: IO String
main = do
    input <- map T.unpack . T.splitOn "," . T.strip <$> T.readFile "16a.input"
    let xs = ['a'..'p']
    print $ foldr exec xs $ reverse input
    return ""
