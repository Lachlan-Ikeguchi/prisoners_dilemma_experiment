-- name: DAMSEL
-- appears weak but can be dangerous

function decide(round, history)
    if round <= 5 then return "cooperate" end
    if history[#history] == "defect" then return "defect" end
    return "cooperate"
end
