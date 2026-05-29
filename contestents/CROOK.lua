-- name: CROOK
-- cheats when possible

function decide(round, history)
    if round == 1 then return "cooperate" end
    if history[#history] == "cooperate" then return "defect" end
    return "cooperate"
end
