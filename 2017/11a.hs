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

move dir (x, y, z)
    | dir == "n"  = (x - 1, y + 1, z)
    | dir == "ne" = (x, y + 1, z - 1)
    | dir == "nw" = (x - 1, y, z + 1)
    | dir == "s"  = (x + 1, y - 1, z)
    | dir == "se" = (x + 1, y, z - 1)
    | dir == "sw" = (x, y - 1, z + 1)
    | otherwise = error ("Invalid direction" ++ dir)

distanceToOrig (x, y, z) = sum [abs x, abs y, abs z] `div` 2

main :: IO String
main = do
    input <- map T.unpack . T.splitOn "," . T.strip <$> T.readFile "11a.input"
    print $ distanceToOrig $ foldr move (0, 0, 0) $ reverse input
    return ""
