-- name: DEMANDING
-- demands cooperation

function decide(round, history)
    if round == 1 then return "defect" end
    if history[#history] == "cooperate" then return "cooperate" end
    return "defect"
end
