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
import Text.Printf (printf)

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

main :: IO String
main = do
    input <- T.unpack . T.strip <$> T.readFile "10a.input"
    let lengths = toAscii input ++ [17, 31, 73, 47, 23]
        (_, _, sparseHash) = foldr (\_ acc -> hashRound lengths acc) (0, 0, [0..255]) [1..64]
        denseHash = map (foldr xor (0 :: Int)) $ split16 sparseHash
    forM_ denseHash (printf "%02x")
    return ""
