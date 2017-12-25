{-# LANGUAGE OverloadedStrings #-}
import Data.Set as S (fromList, size, member, insert, empty)
import Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member)
import Data.Text as T (Text, strip, lines, words, unpack)
import Data.Text.IO as T (readFile)
import Control.Applicative ((<$>))
import Data.List (sort)

parents :: M.Map String String -> [(String, [String])] -> M.Map String String
parents parentMapping childrenMapping = foldr assignParent parentMapping childrenMapping
    where assignParent (parent, children) parentMapping =
            foldr (\child acc -> M.insert child parent acc) parentMapping children

stripComma = filter (/= ',')

main :: IO String
main = do
    input <- map (map T.unpack) . map T.words . T.lines . T.strip <$> T.readFile "7a.input"
    let childrenMapping = map (\x -> (head x, map stripComma $ drop 3 x)) input
        parentMapping = parents M.empty childrenMapping
    print $ fst $ head $ filter (\(child, _) -> not (M.member child parentMapping)) childrenMapping
    return ""
