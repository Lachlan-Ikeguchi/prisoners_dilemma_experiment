-- name: BADGER
-- stubbornly cooperates until first defection, then defects forever

function decide(round, history)
    if round == 1 then return "cooperate" end
    for i = 1, #history do
        if history[i] == "defect" then return "defect" end
    end
    return "cooperate"
end
