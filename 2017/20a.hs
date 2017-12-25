{-# LANGUAGE OverloadedStrings #-}
{-# LANGUAGE BangPatterns #-}
import Control.Applicative ((<$>))
import Control.Monad (forM_)
import Data.Bits (xor, (.&.))
import Data.Char (ord)
import Data.Set as S (fromList, size, member, insert, empty)
import Data.Map.Strict as M (Map, fromList, empty, size, findWithDefault, adjust, insert, member, foldr, foldrWithKey)
import Data.Text as T (Text, strip, lines, words, unpack, splitOn, pack)
import Data.Text.IO as T (readFile)
import Data.List (sortOn, group, foldl', elemIndex, iterate)
import Debug.Trace (trace)
import Numeric (showHex)
import Text.Printf (printf)

(|>) :: a -> (a -> b) -> b
(|>) = flip ($)

readInt :: String -> Int
readInt = read

type Coord = (Int, Int, Int)
data Particle = Particle Coord Coord Coord deriving (Show)

vecAdd :: Coord -> Coord -> Coord
vecAdd (a, b, c) (d, e, f) = (a + d, b + e, c + f)

updateParticle :: Particle -> Particle
updateParticle (Particle pos vel acc) =
    let newVel = vel `vecAdd` acc
        newPos = pos `vecAdd` newVel
    in Particle newPos newVel acc

stripVec :: String -> String
stripVec s = dropWhile (/= '<') s |> drop 1 |> takeWhile (/= '>')

distanceToOrig :: Particle -> Int
distanceToOrig (Particle (x, y, z) _ _) = abs x + abs y + abs z

closestToZero :: [Particle] -> Int -> Int -> Int -> Int
closestToZero [] _ minIdx _ = minIdx
closestToZero (x:particles) minDist minIdx idx =
    if distanceToOrig x < minDist
        then closestToZero particles (distanceToOrig x) idx (idx + 1)
        else closestToZero particles minDist minIdx (idx + 1)

main :: IO ()
main = do
    input <- map (map T.unpack . T.splitOn ", ") . T.lines <$> T.readFile "20a.input"
    let stripped = map (map (T.pack . stripVec)) input
        split = map (map (map (readInt . T.unpack) . T.splitOn ",")) stripped
        coords = map (map (\[a, b, c] -> (a, b, c))) split
        particles = map (\[p, v, a] -> Particle p v a) coords
        simulation = iterate (map updateParticle) particles
    print $ closestToZero (simulation !! 1000) (maxBound :: Int) 0 0
