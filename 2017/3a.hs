data SpiralDirection = DUp | DDown | DLeft | DRight

step dir width idx curr max (x, y) =
    if curr == max
        then (x, y)
        else case dir of
            DUp | idx < width    -> step DUp width (idx + 1) (curr + 1) max (x, y + 1)
            DUp                  -> step DLeft (width + 1) 1 (curr + 1) max (x - 1, y)
            DLeft | idx < width  -> step DLeft width (idx + 1) (curr + 1) max (x - 1, y)
            DLeft                -> step DDown width 1 (curr + 1) max (x, y - 1)
            DDown | idx < width  -> step DDown width (idx + 1) (curr + 1) max (x, y - 1)
            DDown                -> step DRight (width + 1) 1 (curr + 1) max (x + 1, y)
            DRight | idx < width -> step DRight width (idx + 1) (curr + 1) max (x + 1, y)
            DRight               -> step DUp width 1 (curr + 1) max (x, y + 1)

main :: IO String
main = let targetCoord = step DDown 0 0 1 368078 (0, 0) in do
    print $ abs (fst targetCoord) + abs (snd targetCoord)
    return ""
