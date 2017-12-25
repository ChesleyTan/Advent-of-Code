{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BangPatterns #-}
import Data.Set as S (fromList, size, member, insert, empty)
import Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member, foldr, foldrWithKey)
import Data.Text as T (Text, strip, lines, words, unpack)
import Data.Text.IO as T (readFile)
import Control.Applicative ((<$>))
import Data.List (sortOn, group)

sumGroups "" _ totalScore _ = totalScore
sumGroups (x:xs) score totalScore inGarbage
    | not inGarbage && x == '{'     = sumGroups xs (score + 1) totalScore inGarbage
    | not inGarbage && x == '}'     = sumGroups xs (score - 1) (totalScore + score) inGarbage
    | not inGarbage && x == '<'     = sumGroups xs score totalScore True
    | inGarbage && x == '>'         = sumGroups xs score totalScore False
    | inGarbage && x == '!'         = sumGroups (tail xs) score totalScore inGarbage
    | otherwise                     = sumGroups xs score totalScore inGarbage

main :: IO String
main = do
    input <- T.unpack . T.strip <$> T.readFile "9a.input"
    print $ sumGroups input 0 0 False
    return ""
