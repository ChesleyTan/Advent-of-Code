{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BangPatterns #-}
import Control.Applicative ((<$>))
import Control.Monad (forM_)
import Data.Bits (xor, (.&.))
import Data.Char (ord)
import Data.Set as S (fromList, size, member, insert, empty)
import Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member, foldr, foldrWithKey)
import Data.Text as T (Text, strip, lines, words, unpack, splitOn)
import Data.Text.IO as T (readFile)
import Data.List (sortOn, group, foldl', elemIndex, iterate)
import Debug.Trace (trace)
import Numeric (showHex)
import Text.Printf (printf)

data Direction = DUp | DDown | DLeft | DRight deriving (Eq, Show)
type Coord = (Int, Int)
data Pose = Pose Coord Direction deriving (Show)
type Grid = [String]

(|>) :: a -> (a -> b) -> b
(|>) = flip ($)

readInt :: String -> Int
readInt = read

inBounds :: Grid -> Coord -> Bool
inBounds grid (x, y) = not (y < 0 || y >= length grid || x < 0 || x >= length (head grid)) && (grid !! y !! x /= ' ')

getDir :: Pose -> Direction
getDir (Pose _ dir) = dir

getCoord :: Pose -> Coord
getCoord (Pose coord _) = coord

getLRNeighbors :: Grid -> Pose -> [Pose]
getLRNeighbors grid (Pose coord dir)
    | dir == DUp = filter (inBounds grid . getCoord) [Pose (fst coord - 1, snd coord) DLeft, Pose (fst coord + 1, snd coord) DRight]
    | dir == DDown = filter (inBounds grid . getCoord) [Pose (fst coord + 1, snd coord) DRight, Pose (fst coord - 1, snd coord) DLeft]
    | dir == DLeft = filter (inBounds grid . getCoord) [Pose (fst coord, snd coord + 1) DDown, Pose (fst coord, snd coord - 1) DUp]
    | dir == DRight = filter (inBounds grid . getCoord) [Pose (fst coord, snd coord - 1) DUp, Pose (fst coord, snd coord + 1) DDown]


isLetter :: Char -> Bool
isLetter ' ' = False
isLetter '-' = False
isLetter '|' = False
isLetter '+' = False
isLetter _ = True

nextPose :: Grid -> Pose -> Int -> Int
nextPose _ pose steps | trace (show pose ++ " " ++ show steps) False = undefined
nextPose grid pose steps =
    let nextCoord = case pose of
            Pose (x, y) DUp -> (x, y - 1)
            Pose (x, y) DDown -> (x, y + 1)
            Pose (x, y) DLeft -> (x - 1, y)
            Pose (x, y) DRight -> (x + 1, y)
        newSteps = steps + 1
    in if inBounds grid nextCoord
        then trace "inBounds" $ nextPose grid (Pose nextCoord (getDir pose)) newSteps
        else
            let nextPos = getLRNeighbors grid pose |> take 1
            in if null nextPos
                then steps
                else trace (show nextCoord) $ nextPose grid (head nextPos) newSteps

main :: IO ()
main = do
    input <- map T.unpack . T.lines <$> T.readFile "19a.input"
    let startingPose = Pose (13, 0) DDown
    print $ nextPose input startingPose 1
