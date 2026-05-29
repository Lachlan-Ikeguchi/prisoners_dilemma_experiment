-- name: BAT
-- defects at night (rounds 50-100)

function decide(round, history)
    if round > 50 then return "defect" end
    return "cooperate"
end
