{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BangPatterns #-}
import Control.Applicative ((<$>))
import Control.Monad (forM_, forM)
import Data.Bits (xor, (.&.))
import Data.Char (ord)
import Data.Set as S (fromList, size, member, insert, empty)
import Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member, foldr, foldrWithKey, (!))
import Data.Text as T (Text, strip, lines, words, unpack, splitOn, pack)
import Data.Text.IO as T (readFile)
import Data.List (sortOn, group, foldl', elemIndex, iterate, lookup)
import Debug.Trace (trace)
import Numeric (showHex)
import Text.Printf (printf)

data Direction = DUp | DDown | DLeft | DRight deriving (Show, Eq)
data Pose = Pose
    { coord :: (Int, Int)
    , direction :: Direction
    } deriving (Show, Eq)

(|>) :: a -> (a -> b) -> b
(|>) = flip ($)

readInt :: String -> Int
readInt = read

iterateN :: Int -> Int -> (a -> a) -> a -> a
iterateN !total !current !f !a
    | current < total = iterateN total (current + 1) f (f a)
    | otherwise = a

updateCoord :: (Int, Int) -> Direction -> (Int, Int)
updateCoord coord dir
    | dir == DUp = (fst coord, snd coord - 1)
    | dir == DDown = (fst coord, snd coord + 1)
    | dir == DLeft = (fst coord - 1, snd coord)
    | dir == DRight = (fst coord + 1, snd coord)

run :: M.Map (Int, Int) Char -> Pose -> Int -> Int -> Int
run grid pose iteration numInfected
    | iteration >= 10000 = numInfected
    | otherwise =
        let currentCoord = coord pose
            currentNode = M.findWithDefault '.' currentCoord grid
            currentDir = direction pose
            newDirection
                | currentNode == '#' && currentDir == DUp = DRight
                | currentNode == '#' && currentDir == DDown = DLeft
                | currentNode == '#' && currentDir == DLeft = DUp
                | currentNode == '#' && currentDir == DRight = DDown
                | currentDir == DUp = DLeft
                | currentDir == DDown = DRight
                | currentDir == DLeft = DDown
                | currentDir == DRight = DUp
            newStatus
                | currentNode == '#' = '.'
                | otherwise = '#'
            newGrid = M.insert currentCoord newStatus grid
            newCoord = updateCoord currentCoord newDirection
            newPose = Pose {coord = newCoord, direction = newDirection}
        in run newGrid newPose (iteration + 1) (if newStatus == '#' then numInfected + 1 else numInfected)

main :: IO ()
main = do
    input <- map T.unpack . T.lines <$> T.readFile "22a.input"
    let width = length input - 1
        grid = M.fromList [((x, y), input !! y !! x) | x <- [0..width], y <- [0..width]]
        start = Pose {coord = (width `div` 2, width `div` 2), direction = DUp}
    print $ run grid start 0 0
