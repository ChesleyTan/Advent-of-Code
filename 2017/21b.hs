{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BangPatterns #-}
import Control.Applicative ((<$>))
import Control.Monad (forM_, forM)
import Data.Bits (xor, (.&.))
import Data.Char (ord)
import Data.Set as S (fromList, size, member, insert, empty)
import Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member, foldr, foldrWithKey)
import Data.Maybe (fromJust)
import Data.Text as T (Text, strip, lines, words, unpack, splitOn, pack)
import Data.Text.IO as T (readFile)
import Data.List (sortOn, group, foldl', elemIndex, iterate, lookup)
import Debug.Trace (trace)
import Numeric (showHex)
import Text.Printf (printf)

(|>) :: a -> (a -> b) -> b
(|>) = flip ($)

readInt :: String -> Int
readInt = read

data Two = Two String String deriving (Show)
data Three = Three String String String deriving (Show)
data Four = Four String String String String deriving (Show, Eq)

instance Eq Two where
    Two [a, b] [c, d] == q =
        let equiv = equivalences2 q
        in any (\(Two [a', b'] [c', d']) -> a' == a && b' == b && c' == c && d' == d) equiv

instance Eq Three where
    Three [a, b, c] [d, e, f] [g, h, i] == q =
        let equiv = equivalences3 q
        in any (\(Three [a', b', c'] [d', e', f'] [g', h', i']) ->
            a' == a && b' == b && c' == c &&
            d' == d && e' == e && f' == f &&
            g' == g && h' == h && i' == i) equiv

parseLine :: [[Text]] -> ([String], [String])
parseLine [x, y] = (map T.unpack x, map T.unpack y)
parseLine _ = error "invalid input"

parseMapping2 :: ([String], [String]) -> Maybe (Two, Three)
parseMapping2 ([[a, b], [c, d]], [[a', b', c'], [d', e', f'], [g', h', i']]) =
    Just ( Two [a, b] [c, d]
         , Three [a', b', c'] [d', e', f'] [g', h', i']
         )
parseMapping2 _ = Nothing

parseMapping3 :: ([String], [String]) -> Maybe (Three, Four)
parseMapping3 ([[a, b, c], [d, e, f], [g, h, i]], [a', b', c', d']) =
    Just ( Three [a, b, c] [d, e, f] [g, h, i]
         , Four a' b' c' d'
         )
parseMapping3 _ = Nothing

equivalences2 :: Two -> [Two]
equivalences2 (Two [a, b] [c, d]) =
    [ Two [a, b] [c, d]
    , Two [b, a] [d, c]
    , Two [c, a] [d, b]
    , Two [a, c] [b, d]
    , Two [d, c] [b, a]
    , Two [c, d] [a, b]
    , Two [b, d] [a, c]
    , Two [d, b] [c, a]
    ]

equivalences3 :: Three -> [Three]
equivalences3 (Three [a, b, c] [d, e, f] [g, h, i]) =
    [ Three [a, b, c] [d, e, f] [g, h, i]
    , Three [c, b, a] [f, e, d] [i, h, g]
    , Three [g, d, a] [h, e, b] [i, f, c]
    , Three [a, d, g] [b, e, h] [c, f, i]
    , Three [i, h, g] [f, e, d] [c, b, a]
    , Three [g, h, i] [d, e, f] [a, b, c]
    , Three [c, f, i] [b, e, h] [a, d, g]
    , Three [i, f, c] [h, e, b] [g, d, a]
    ]

toMatrix2 :: [String] -> [[Two]]
toMatrix2 rows =
    let end = length rows - 1
    in head $
        forM [0,2..end] $ \i ->
            let {row = rows !! i; nextRow = rows !! (i + 1)} in
            forM [0,2..end] $ \j ->
                [Two [row !! j, row !! (j + 1)] [nextRow !! j, nextRow !! (j + 1)]]

toMatrix3 :: [String] -> [[Three]]
toMatrix3 rows =
    let end = length rows - 1
    in head $
        forM [0,3..end] $ \i ->
            let {row = rows !! i; nextRow = rows !! (i + 1); nextNextRow = rows !! (i + 2)} in
            forM [0,3..end] $ \j ->
                [Three [row !! j, row !! (j + 1), row !! (j + 2)]
                       [nextRow !! j, nextRow !! (j + 1), nextRow !! (j + 2)]
                       [nextNextRow !! j, nextNextRow !! (j + 1), nextNextRow !! (j + 2)]
                ]

unMatrix3 :: [[Three]] -> [String]
unMatrix3 rows =
    let processRow (acc1, acc2, acc3) row = case row of
            Three a b c:xs ->
                processRow ( acc1 ++ a
                           , acc2 ++ b
                           , acc3 ++ c
                           ) xs
            [] -> [acc1, acc2, acc3]
    in concatMap (processRow ([], [], [])) rows

unMatrix4 :: [[Four]] -> [String]
unMatrix4 rows =
    let processRow (acc1, acc2, acc3, acc4) row = case row of
            Four a b c d:xs ->
                processRow ( acc1 ++ a
                           , acc2 ++ b
                           , acc3 ++ c
                           , acc4 ++ d
                           ) xs
            [] -> [acc1, acc2, acc3, acc4]
    in concatMap (processRow ([], [], [], [])) rows

transform2 :: [(Two, Three)] -> Two -> Three
--transform2 mapping x | trace (show mapping ++ ":" ++ show x) False = undefined
transform2 mapping x = fromJust $ lookup x mapping

transform3 :: [(Three, Four)] -> Three -> Four
--transform3 mapping x | trace (show mapping ++ ":" ++ show x) False = undefined
transform3 mapping x = fromJust $ lookup x mapping

transform :: [String] -> [(Two, Three)] -> [(Three, Four)] -> [String]
transform grid mappings2 mappings3
    | length grid `mod` 2 == 0 =
        let matrix2 = toMatrix2 grid
        in map (map (transform2 mappings2)) matrix2 |> unMatrix3
    | otherwise =
        let matrix3 = toMatrix3 grid
        in map (map (transform3 mappings3)) matrix3 |> unMatrix4

iterateN :: Int -> Int -> (a -> a) -> a -> a
iterateN !total !current !f !a
    | current < total = iterateN total (current + 1) f (f a)
    | otherwise = a

countOn :: [String] -> Int
countOn xs = concat xs |> filter (== '#') |> length

-- Can be optimized by pre-computing all equivalences for the input rules instead of dynamically
-- calculating equivalences
main :: IO ()
main = do
    input <- map (map (T.splitOn "/") . T.splitOn " => ") . T.lines <$> T.readFile "21a.input"
    let parsedInput = map parseLine input
        mappings2 = map parseMapping2 parsedInput |> filter (/= Nothing) |> map fromJust
        mappings3 = map parseMapping3 parsedInput |> filter (/= Nothing) |> map fromJust
    print $ parsedInput
    print $ mappings2
    --print $ toMatrix3 ["011011", "011011", "011011", "011011", "011011", "011011"]
    --print $ unMatrix4 [[Four "0110" "0110" "0110" "0110", Four "0110" "0110" "0110" "0110"], [Four "0110" "0110" "0110" "0110", Four "0110" "0110" "0110" "0110"]]
    --print $ iterateN 18 0 (\x -> transform x mappings2 mappings3) [".#.", "..#", "###"]
    print $ countOn $ iterateN 18 0 (\x -> transform x mappings2 mappings3) [".#.", "..#", "###"]
