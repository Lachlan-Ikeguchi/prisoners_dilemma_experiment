-- name: HIT AND RUN
-- defects once then cooperates twice, repeating

function decide(round, history)
    local cycle = (round - 1) % 3
    if cycle == 0 then
        return "defect"
    else
        return "cooperate"
    end
end
