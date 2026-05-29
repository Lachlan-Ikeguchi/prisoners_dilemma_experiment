-- name: DOWNRIGHT MEAN
-- defects on rounds that are multiples of 3

function decide(round, history)
    if round % 3 == 0 then
        return "defect"
    else
        return "cooperate"
    end
end
