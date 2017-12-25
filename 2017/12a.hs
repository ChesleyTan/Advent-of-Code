{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BangPatterns #-}
import Control.Monad (forM_)
import Data.Bits (xor)
import Data.Char (ord)
import Data.Set as S (fromList, size, member, insert, empty)
import qualified Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member, foldr, foldrWithKey)
import Data.Text as T (Text, strip, lines, words, unpack, pack, splitOn)
import Data.Text.IO as T (readFile)
import Control.Applicative ((<$>))
import Data.List (sortOn, group, foldl')
import Text.Printf (printf)
import Debug.Trace (trace)

(|>) = flip ($)
readInt = read :: String -> Int

parseLine line =
    let lineStr = T.unpack line
        node = takeWhile (/= ' ') lineStr |> readInt
        edgesList = dropWhile (/= '>') lineStr |> drop 1
        edges = T.pack edgesList |> T.splitOn ", " |> map (readInt . T.unpack)
    in (node, edges)

dfs node (visited, graph)
    | S.member node visited = (visited, graph)
    | otherwise =
        let newVisited = S.insert node visited
        in foldr dfs (newVisited, graph) $ M.findWithDefault [] node graph

main :: IO String
main = do
    input <- map parseLine . T.lines . T.strip <$> T.readFile "12a.input"
    let graph = M.fromList input
    print $ S.size $ fst $ dfs 0 (S.empty, graph)
    return ""
