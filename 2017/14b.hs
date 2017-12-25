{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BangPatterns #-}
import Control.Monad (forM_)
import Data.Bits (xor)
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

reverse' xs length = (reverse $ take length xs) ++ (drop length xs)

transform (position, skipSize, xs) length  =
    let circularXs = (drop position xs) ++ (take position xs)
        reversedXs = reverse' circularXs length
        inversePosition = Prelude.length xs - position
        normalizedXs = (drop inversePosition reversedXs) ++ (take inversePosition reversedXs)
    in ((position + skipSize + length) `mod` (Prelude.length xs), skipSize + 1, normalizedXs)

hashRound lengths (position, skipSize, xs) = foldl' transform (position, skipSize, xs) lengths

toAscii = map ord

split16 [] = []
split16 !xs = (take 16 xs) : split16 (drop 16 xs)

toHex x = showHex x "" |> (\result -> if length result < 2 then "0" ++ result else result)

knotHash :: String -> String
knotHash key =
    let lengths = toAscii key ++ [17, 31, 73, 47, 23]
        (_, _, sparseHash) = foldr (\_ acc -> hashRound lengths acc) (0, 0, [0..255]) [1..64]
        denseHash = map (foldr xor (0 :: Int)) $ split16 sparseHash
    in map toHex denseHash |> concat

genKeys key = [key ++ "-" ++ (show idx) | idx <- [0..127]]

update :: Int -> a -> [a] -> [a]
update idx newValue xs = (take idx xs) ++ [newValue] ++ (drop (idx + 1) xs)

updateGrid :: (Int, Int) -> a -> [[a]] -> [[a]]
updateGrid (x, y) newValue grid = update y (update x newValue $ grid !! y) grid

flood (x, y) grid
    | x < 0 || y < 0 = grid
    | x >= length grid || y >= length (head grid) = grid
    | grid !! y !! x /= '1' = grid
    | otherwise =
        let newGrid = updateGrid (x, y) 'x' grid
        in foldr flood newGrid [(x-1, y), (x+1, y), (x, y-1), (x, y+1)]

hexToBin :: Char -> String
hexToBin x
    | x == '0' = "0000"
    | x == '1' = "0001"
    | x == '2' = "0010"
    | x == '3' = "0011"
    | x == '4' = "0100"
    | x == '5' = "0101"
    | x == '6' = "0110"
    | x == '7' = "0111"
    | x == '8' = "1000"
    | x == '9' = "1001"
    | x == 'a' = "1010"
    | x == 'b' = "1011"
    | x == 'c' = "1100"
    | x == 'd' = "1101"
    | x == 'e' = "1110"
    | x == 'f' = "1111"
    | otherwise = error "invalid hex"

-- Instead of flood fill, could also DFS and count number of nodes that haven't been visited by a
-- prior DFS
countRegions grid =
    let coords = [(x, y) | x <- [0..length (head grid) - 1], y <- [0..length grid - 1]]
    in foldr (\(x, y) (grid, count) -> (flood (x, y) grid, count + (if grid !! y !! x == '1' then 1 else 0))) (grid, 0) coords

main :: IO String
main = do
    let keys = genKeys "nbysizxe"
        grid = map (concat . map hexToBin . knotHash) keys
    --print $ countRegions ["101", "001", "001"]
    print $ countRegions grid
    return ""
