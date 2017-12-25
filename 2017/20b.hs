
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

instance Eq Particle where
    Particle a _ _ == Particle b _ _ = a == b

vecAdd :: Coord -> Coord -> Coord
vecAdd (a, b, c) (d, e, f) = (a + d, b + e, c + f)

updateParticle :: Particle -> Particle
updateParticle (Particle pos vel acc) =
    let newVel = vel `vecAdd` acc
        newPos = pos `vecAdd` newVel
    in Particle newPos newVel acc

stripVec :: String -> String
stripVec s = dropWhile (/= '<') s |> drop 1 |> takeWhile (/= '>')

removeCollisions :: [Particle] -> [Particle] -> [Particle]
removeCollisions acc [] = acc
removeCollisions acc [x] = x:acc
removeCollisions acc (x:y:particles)
    | x == y = removeCollisions acc (dropWhile (== x) particles)
    | otherwise = removeCollisions (x:acc) (y:particles)

main :: IO ()
main = do
    input <- map (map T.unpack . T.splitOn ", ") . T.lines <$> T.readFile "20a.input"
    let stripped = map (map (T.pack . stripVec)) input
        split = map (map (map (readInt . T.unpack) . T.splitOn ",")) stripped
        coords = map (map (\[a, b, c] -> (a, b, c))) split
        particles = map (\[p, v, a] -> Particle p v a) coords
        simulation = iterate (removeCollisions [] . sortOn (\(Particle pos _ _) -> pos) . map updateParticle) particles
    print $ length $ (simulation !! 100)
