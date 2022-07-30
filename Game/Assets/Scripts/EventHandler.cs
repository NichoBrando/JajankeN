using System.Text;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using System.Runtime.InteropServices;
using TMPro;

public class EventHandler : MonoBehaviour
{
    private Dictionary<string, int> playerScores = new Dictionary<string, int>();
    public Dictionary<string, Image> movementImages = new Dictionary<string, Image>();

    [SerializeField]
    private TextMeshProUGUI scoreLabel;

    [SerializeField]
    private TextMeshProUGUI winnerLabel;

    [SerializeField]
    private GameObject winnerPanel;

    private string currentMovement;

    [SerializeField]
    private Image enemyMovementImage;
    [SerializeField]
    private Image playerMovementImage;
    private string localPlayerName = "";
    private string matchState = "attack";

    [DllImport("__Internal")]
    private static extern void updateSelectedMovement(string selectedMovement);

    [DllImport("__Internal")]
    public static extern void onLeaveMatch();

    private void Start ()
    {
        movementImages.Add("rock", GameObject.Find("Rock").GetComponent<Image>());
        movementImages.Add("paper", GameObject.Find("Paper").GetComponent<Image>());
        movementImages.Add("scissor", GameObject.Find("Scissor").GetComponent<Image>());
        # if UNITY_EDITOR
            OnStartMatch("player1,player2");
        # endif
    }

    private void UpdateScoreLabel()
    {
        StringBuilder newScoreLabel = new StringBuilder();
        foreach (KeyValuePair<string, int> entry in playerScores)
        {
            if (newScoreLabel.Length == 0)
            {
                newScoreLabel.Append($"{entry.Key} {entry.Value} X ");
            }
            else
            {
                newScoreLabel.Append($"{entry.Value} {entry.Key}");
            }
        }
        scoreLabel.text = newScoreLabel.ToString();
    }

    public void OnStartMatch(string players)
    {
        string[] playerNames = players.Split(',');
        playerScores = new Dictionary<string, int>();
        foreach (string playerName in playerNames)
        {
            playerScores.Add(playerName, 0);
        }
        UpdateScoreLabel();
    }

    public void OnSelect (string movement) 
    {
        if (matchState != "attack")
        {
            return;
        }
        if (currentMovement != null)
        {
            movementImages[currentMovement].color = Color.white;
            if (movement == currentMovement) 
            {
                currentMovement = null;
                return;
            }
        }
        currentMovement = movement;
        movementImages[currentMovement].color = Color.red;
        # if UNITY_EDITOR
        ShowMovements("rock");
        OnScore("player1");
        # else
        updateSelectedMovement(movement);
        # endif
    }

    public void OnEnemySelection () 
    {
        enemyMovementImage.color = Color.red;
    }

    public void OnEnemyCancelSelection () 
    {
        enemyMovementImage.color = Color.white;
    }

    public void ShowMovements (string enemyMovement)
    {
        matchState = "waiting";
        playerMovementImage.sprite = movementImages[currentMovement].sprite;
        enemyMovementImage.sprite = movementImages[enemyMovement].sprite;
    }

    public void OnScore (string playerName)
    {
        playerScores[playerName]++;
        UpdateScoreLabel();
        if (localPlayerName == playerName)
        {
            playerMovementImage.color = Color.green;
        }
        else
        {
            enemyMovementImage.color = Color.green;
        }
        if (playerScores[playerName] == 3)
        {
            StartCoroutine(OnEndMatch(playerName));
            OnEndMatch(playerName);
            return;
        }
        StartCoroutine("StartNextRound");
    }

    private IEnumerator OnEndMatch(string playerName)
    {
        yield return new WaitForSeconds(1);
        movementImages[currentMovement].color = Color.white;
        currentMovement = null;
        matchState = "finished";
        string message = $"{(playerName == localPlayerName ? "you" : playerName)} won!";
        winnerLabel.text = message;
        winnerPanel.SetActive(true);
    }

    private IEnumerator StartNextRound()
    {
        yield return new WaitForSeconds(1);
        movementImages[currentMovement].color = Color.white;
        currentMovement = null;
        matchState = "attack";
        enemyMovementImage.color = Color.white;
        playerMovementImage.color = Color.white;
    }
}
