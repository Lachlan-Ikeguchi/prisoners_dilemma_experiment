-- name: ANTI CYCLIC
-- cycles through defect, defect, cooperate pattern

function decide(round, history)
    local cycle = (round - 1) % 3
    if cycle < 2 then
        return "defect"
    else
        return "cooperate"
    end
end
