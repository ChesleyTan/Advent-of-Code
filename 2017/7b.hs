{-# LANGUAGE OverloadedStrings #-}
import Data.Set as S (fromList, size, member, insert, empty)
import Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member, foldrWithKey)
import Data.Text as T (Text, strip, lines, words, unpack)
import Data.Text.IO as T (readFile)
import Control.Applicative ((<$>))
import Data.List (sortOn, group)

parents :: M.Map String String -> [(String, [String])] -> M.Map String String
parents = foldr assignParent
    where assignParent (parent, children) parentMapping =
            foldr (\child acc -> M.insert child parent acc) parentMapping children

stripComma = filter (/= ',')
stripParens = filter (\x -> x /= '(' && x /= ')')
readInt = read :: String -> Int

subtreeWeight childrenMapping weightMapping parent =
    M.findWithDefault 0 parent weightMapping + sum (map (subtreeWeight childrenMapping weightMapping) childrenOfParent)
    where childrenOfParent = M.findWithDefault [] parent childrenMapping

outlier xs = fst $ head $ sortOn (\(_, b) -> length $ filter ((== b) . snd) xs) xs

isBalanced :: M.Map String [String] -> M.Map String Int -> String -> Bool
isBalanced childrenMapping weightMapping parent =
    1 == length (group $ map (subtreeWeight childrenMapping weightMapping) $ M.findWithDefault [] parent childrenMapping)

getHighestUnbalanced childrenMapping weightMapping parent
    | not (isBalanced childrenMapping weightMapping parent) =
        let childrenBalances = map (\child -> (child, isBalanced childrenMapping weightMapping child)) childrenOfParent
        in if all snd childrenBalances
            then outlier $ map (\child -> (child, subtreeWeight childrenMapping weightMapping child)) childrenOfParent
            else getHighestUnbalanced childrenMapping weightMapping (fst $ head $ filter (not . snd) childrenBalances)
    | otherwise = parent
    where childrenOfParent = M.findWithDefault [] parent childrenMapping

siblings child childrenMapping parentMapping =
    let parent = M.findWithDefault "" child parentMapping
    in M.findWithDefault [] parent childrenMapping

main :: IO String
main = do
    input <- (map (map T.unpack . T.words) . T.lines . T.strip) <$> T.readFile "7a.input"
    let assocChildrenMapping = map (\x -> (head x, map stripComma $ drop 3 x)) input
        childrenMapping = M.fromList assocChildrenMapping
        weightMapping = M.fromList $ map (\x -> (head x, readInt $ stripParens $ head $ tail x)) input
        parentMapping = parents M.empty assocChildrenMapping
        root = fst $ head $ filter (\(child, _) -> not (M.member child parentMapping)) assocChildrenMapping
        unbalancedNode = getHighestUnbalanced childrenMapping weightMapping root
        unbalancedNodeSiblingWeights = map (subtreeWeight childrenMapping weightMapping) $ siblings unbalancedNode childrenMapping parentMapping
        weightDiff = maximum unbalancedNodeSiblingWeights - minimum unbalancedNodeSiblingWeights
    print $ flip (-) weightDiff $ M.findWithDefault 0 unbalancedNode weightMapping
    return ""
