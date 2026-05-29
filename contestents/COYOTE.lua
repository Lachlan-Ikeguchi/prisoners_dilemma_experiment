-- name: COYOTE
-- tricky: defects, cooperates, defects

function decide(round, history)
    local cycle = (round - 1) % 3
    if cycle == 0 then return "defect"
    elseif cycle == 1 then return "cooperate"
    else return "defect" end
end
