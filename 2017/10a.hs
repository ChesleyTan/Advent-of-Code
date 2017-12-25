{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BangPatterns #-}
import Data.Set as S (fromList, size, member, insert, empty)
import Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member, foldr, foldrWithKey)
import Data.Text as T (Text, strip, lines, words, unpack, splitOn)
import Data.Text.IO as T (readFile)
import Control.Applicative ((<$>))
import Data.List (sortOn, group, foldl')

reverse' xs length = (reverse $ take length xs) ++ (drop length xs)

transform (position, skipSize, xs) length  =
    let circularXs = (drop position xs) ++ (take position xs)
        reversedXs = reverse' circularXs length
        inversePosition = Prelude.length xs - position
        normalizedXs = (drop inversePosition reversedXs) ++ (take inversePosition reversedXs)
    in ((position + skipSize + length) `mod` (Prelude.length xs), skipSize + 1, normalizedXs)

main :: IO String
main = do
    input <- map ((read :: String -> Int) . T.unpack) . T.splitOn "," . T.strip <$> T.readFile "10a.input"
    let (_, _, result) = foldl' transform (0, 0, [0..255]) input
    print $ (head result) * (head $ tail result)
    return ""
