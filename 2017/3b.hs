import Data.Map.Strict as M
data SpiralDirection = DUp | DDown | DLeft | DRight

calculate (x, y) memo =
    let sumAdjacent = M.findWithDefault 0 (x - 1, y - 1) memo +
                      M.findWithDefault 0 (x - 1, y) memo +
                      M.findWithDefault 0 (x - 1, y + 1) memo +
                      M.findWithDefault 0 (x, y - 1) memo +
                      M.findWithDefault 0 (x, y + 1) memo +
                      M.findWithDefault 0 (x + 1, y - 1) memo +
                      M.findWithDefault 0 (x + 1, y) memo +
                      M.findWithDefault 0 (x + 1, y + 1) memo
    in M.insert (x, y) sumAdjacent memo

step dir width idx max (x, y) memo =
    if M.findWithDefault 0 (x, y) memo > max
        then M.findWithDefault 0 (x, y) memo
        else case dir of
            DUp | idx < width    -> step DUp width (idx + 1) max (x, y + 1) (calculate (x, y + 1) memo)
            DUp                  -> step DLeft (width + 1) 1 max (x - 1, y) (calculate (x - 1, y) memo)
            DLeft | idx < width  -> step DLeft width (idx + 1) max (x - 1, y) (calculate (x - 1, y) memo)
            DLeft                -> step DDown width 1 max (x, y - 1) (calculate (x, y - 1) memo)
            DDown | idx < width  -> step DDown width (idx + 1) max (x, y - 1) (calculate (x, y - 1) memo)
            DDown                -> step DRight (width + 1) 1 max (x + 1, y) (calculate (x + 1, y) memo)
            DRight | idx < width -> step DRight width (idx + 1) max (x + 1, y) (calculate (x + 1, y) memo)
            DRight               -> step DUp width 1 max (x, y + 1) (calculate (x, y + 1) memo)

main :: IO String
main = let ans = step DDown 0 0 368078 (0, 0) (M.fromList [((0, 0), 1)]) in do
    print ans
    return ""
